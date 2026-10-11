<?php

declare(strict_types=1);

namespace Slackblocks\Examples;

use Psr\Http\Client\ClientInterface;
use Psr\Http\Message\{RequestInterface, ResponseInterface};

/** Example PSR-18 middleware: retry only explicit 429s, at most twice. */
final readonly class RateLimitedClient implements ClientInterface
{
    public function __construct(private ClientInterface $client) {}

    public function sendRequest(RequestInterface $request): ResponseInterface
    {
        for ($attempt = 0; $attempt < 3; ++$attempt) {
            $response = $this->client->sendRequest($request);
            if ($response->getStatusCode() !== 429) {
                return $response;
            }
            $delay = $response->getHeaderLine('Retry-After');
            // Refuse long/missing delays; callers can schedule a later retry.
            if ($attempt === 2 || preg_match('/\A[01]\z/', $delay) !== 1) {
                throw new \RuntimeException('Slack rate limit: retry budget exhausted or Retry-After exceeds one second');
            }
            if ($delay === '1') {
                usleep(1_000_000);
            }
        }
        throw new \LogicException('unreachable');
    }
}
