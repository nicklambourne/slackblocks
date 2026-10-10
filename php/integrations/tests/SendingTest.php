<?php

declare(strict_types=1);

namespace Slackblocks\Examples\Tests;

use Illuminate\Http\Client\Factory;
use Illuminate\Notifications\{Notification, Slack\SlackChannel};
use Nyholm\Psr7\Uri;
use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;
use Psr\Http\Client\ClientInterface;
use Psr\Http\Message\{RequestInterface, ResponseInterface};
use Slackblocks as S;
use Slackblocks\Examples\{LaravelTemplate, Sending};
use Symfony\Component\HttpClient\{HttpClient, Psr18Client};

final class SendingTest extends TestCase
{
    private static string $directory;
    private static string $endpoint;
    private static mixed $server;

    public static function setUpBeforeClass(): void
    {
        self::$directory = sys_get_temp_dir() . '/slackblocks-http-' . bin2hex(random_bytes(6));
        mkdir(self::$directory);
        $socket = stream_socket_server('tcp://127.0.0.1:0', $error, $message);
        if ($socket === false) {
            throw new \RuntimeException($message);
        }
        $address = stream_socket_get_name($socket, false);
        fclose($socket);
        self::$endpoint = 'http://' . $address . '/api/chat.postMessage';
        $environment = getenv();
        $environment['SLACKBLOCKS_MOCK_DIR'] = self::$directory;
        self::$server = proc_open([PHP_BINARY,'-S',$address,__DIR__ . '/router.php'], [['pipe','r'],['file',self::$directory . '/server.log','a'],['file',self::$directory . '/server.log','a']], $pipes, null, $environment);
        if (!is_resource(self::$server)) {
            throw new \RuntimeException('mock server failed to start');
        }
        fclose($pipes[0]);
        for ($attempt = 0;$attempt < 100;++$attempt) {
            $connection = @stream_socket_client('tcp://' . $address, $error, $message, .05);
            if ($connection !== false) {
                fclose($connection);
                return;
            }
            usleep(20_000);
        }
        throw new \RuntimeException('mock server did not become ready');
    }

    public static function tearDownAfterClass(): void
    {
        if (is_resource(self::$server)) {
            proc_terminate(self::$server);
            proc_close(self::$server);
        }
        foreach (glob(self::$directory . '/*') as $file) {
            unlink($file);
        }
        rmdir(self::$directory);
    }

    protected function setUp(): void
    {
        file_put_contents(self::$directory . '/case', 'success');
        file_put_contents(self::$directory . '/requests', '');
    }

    private static function payload(): S\MessagePayload
    {
        return new S\MessagePayload(channel: 'C123', text: 'Fallback', mrkdwn: false, unfurlLinks: false, blocks: [
            new S\SectionBlock(text: '*Modern*', accessory: new S\ButtonElement(text: 'View', actionId: 'view')),
            new S\DataTableBlock(rows: [[new S\RawText('Count')],[new S\RawNumber(9007199254740993, 'Exact')]], caption: 'Counts'),
            new S\PlanBlock(title: 'Build', tasks: [new S\TaskCardBlock(taskId: '1', title: 'Pending', status: S\TaskStatus::Pending)]),
        ], attachments: [new S\Attachment([new S\SectionBlock('Attachment')], color: S\Color::GOOD)], metadata: new S\JsonObject(['event_type' => 'test','event_payload' => (object) []]));
    }

    private static function client(int $timeoutMs = 5000): ClientInterface
    {
        // Only tests rewrite the SDK's Slack endpoint; fake tokens never leave loopback.
        return new class (new Psr18Client(HttpClient::create(['timeout' => $timeoutMs / 1000,'max_duration' => $timeoutMs / 1000,'max_redirects' => 0])), self::$endpoint) implements ClientInterface {
            public function __construct(private ClientInterface $client, private string $endpoint) {}
            public function sendRequest(RequestInterface $request): ResponseInterface
            {
                return $this->client->sendRequest($request->withUri(new Uri($this->endpoint)));
            }
        };
    }

    /** @return iterable<string,array{string}> */
    public static function transports(): iterable
    {
        yield 'cURL' => ['curl'];
        yield 'JoliCode' => ['jolicode'];
    }

    private static function send(string $transport, int $timeoutMs = 5000): array
    {
        return $transport === 'curl' ? Sending::curl(self::payload(), 'xoxb-test-only', self::$endpoint, $timeoutMs) : Sending::jolicode(self::payload(), 'xoxb-test-only', self::client($timeoutMs));
    }

    private static function requests(): array
    {
        return array_map(static fn(string $line): array => json_decode($line, true, flags: JSON_THROW_ON_ERROR), file(self::$directory . '/requests', FILE_IGNORE_NEW_LINES));
    }

    #[DataProvider('transports')]
    public function testCompletePayloadPreserved(string $transport): void
    {
        self::assertTrue(self::send($transport)['ok']);
        $requests = self::requests();
        self::assertCount(1, $requests);
        $request = $requests[0];
        self::assertSame('/api/chat.postMessage', $request['path']);
        $headers = array_change_key_case($request['headers']);
        self::assertSame('Bearer xoxb-test-only', $headers['authorization']);
        $expected = json_decode(self::payload()->toJson());
        if ($transport === 'curl') {
            self::assertEquals($expected, json_decode($request['body']));
        } else {
            parse_str($request['body'], $form);
            foreach (['blocks','attachments','metadata'] as $key) {
                self::assertEquals($expected->$key, json_decode($form[$key]));
            }
            self::assertSame('0', $form['mrkdwn']);
            self::assertSame('0', $form['unfurl_links']);
            self::assertSame($expected->channel, $form['channel']);
            self::assertSame($expected->text, $form['text']);
        }
    }

    #[DataProvider('transports')]
    public function testDocumentedSendingEntryPoint(string $transport): void
    {
        $send = require __DIR__ . '/../examples/' . $transport . '.php';
        $result = $send('xoxb-test-only', $transport === 'curl' ? self::$endpoint : self::client());
        self::assertTrue($result['ok']);
        self::assertCount(1, self::requests());
        $request = self::requests()[0];
        self::assertSame('Bearer xoxb-test-only', array_change_key_case($request['headers'])['authorization']);
        if ($transport === 'curl') {
            $body = json_decode($request['body'], true, flags: JSON_THROW_ON_ERROR);
        } else {
            parse_str($request['body'], $body);
            $body['blocks'] = json_decode($body['blocks'], true, flags: JSON_THROW_ON_ERROR);
        }
        self::assertSame('C0123456789', $body['channel']);
        self::assertSame('Hello', $body['text']);
        self::assertSame('*Hello* from PHP', $body['blocks'][0]['text']['text']);
    }

    #[DataProvider('transports')]
    public function testBoundedRateLimitRetry(string $transport): void
    {
        file_put_contents(self::$directory . '/case', 'retry');
        self::assertTrue(self::send($transport)['ok']);
        self::assertCount(3, self::requests());
        self::assertCount(1, array_unique(array_column(self::requests(), 'body')));
    }

    /** @return iterable<string,array{string,string,int}> */
    public static function failures(): iterable
    {
        foreach (['curl','jolicode'] as $transport) {
            foreach (['api' => 1,'http' => 1,'malformed' => 1,'redirect' => 1,'limit' => 3,'long-limit' => 1,'delay' => 1] as $case => $count) {
                yield $transport . ' ' . $case => [$transport,$case,$count];
            }
        }
    }

    #[DataProvider('failures')]
    public function testFailuresAreNotReportedAsSuccess(string $transport, string $case, int $count): void
    {
        file_put_contents(self::$directory . '/case', $case);
        $error = null;
        $start = microtime(true);
        try {
            self::send($transport, $case === 'delay' ? 50 : 5000);
        } catch (\Throwable $e) {
            $error = $e;
        }
        self::assertNotNull($error, 'expected transport or API failure');
        self::assertNotInstanceOf(\Error::class, $error, 'A programming error must not satisfy a transport failure test');
        if ($case === 'api') {
            self::assertStringContainsString('channel_not_found', $error->getMessage());
        }
        if ($case === 'limit' || $case === 'long-limit') {
            self::assertStringContainsString('rate limit', $error->getMessage());
        }
        if ($case === 'malformed') {
            self::assertInstanceOf(\JsonException::class, $error);
        }
        self::assertLessThan(3, microtime(true) - $start);
        self::assertCount($count, self::requests(), 'redirects or retries must stay bounded');
        if ($case === 'delay') {
            usleep(350_000);
        }
    }

    /** @return iterable<string,array{bool}> */
    public static function laravelMessages(): iterable
    {
        yield 'complete model payload' => [false];
        yield 'documented entry point' => [true];
    }

    #[DataProvider('laravelMessages')]
    public function testLaravelRunsItsActualNotificationChannel(bool $documented): void
    {
        $payload = $documented
            ? new S\MessagePayload(channel: 'C0123456789', text: 'Hello', blocks: [new S\SectionBlock('*Hello* from Laravel')])
            : self::payload();
        $message = $documented
            ? require __DIR__ . '/../examples/laravel.php'
            : LaravelTemplate::message($payload->channel, $payload->text, $payload->blocks)->unfurlLinks(false);
        self::assertSame($payload->channel, $message->toArray()['channel']);
        $http = new Factory();
        $http->fake(['https://slack.com/api/chat.postMessage' => $http->response(['ok' => true], 200)]);
        $notification = new class ($message) extends Notification {
            public function __construct(private \Illuminate\Notifications\Slack\SlackMessage $message) {}
            public function toSlack(mixed $notifiable): \Illuminate\Notifications\Slack\SlackMessage
            {
                return $this->message;
            }
        };
        $notifiable = new class {
            public function routeNotificationFor(string $driver, mixed $notification): \Illuminate\Notifications\Slack\SlackRoute
            {
                return \Illuminate\Notifications\Slack\SlackRoute::make('C123', 'xoxb-test-only');
            }
        };
        (new SlackChannel($http))->send($notifiable, $notification);
        $http->assertSent(function ($request) use ($payload): bool {
            self::assertSame('Bearer xoxb-test-only', $request->header('Authorization')[0]);
            self::assertSame('C123', $request['channel'], 'Laravel notification routing overrides the message channel');
            self::assertFalse($request['unfurl_links']);
            self::assertEquals(json_decode(json_encode($payload->toArray()['blocks'])), json_decode(json_encode($request['blocks'])));
            self::assertSame($payload->text, $request['text']);
            return true;
        });
    }

    public function testLaravelTemplateRefusesLossyEmptyObjects(): void
    {
        $this->expectException(\LogicException::class);
        LaravelTemplate::message('C1', 'Fallback', [new S\SectionBlock('x', extensions: new S\JsonObject(['app' => (object) []]))]);
    }
}
