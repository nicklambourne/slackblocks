import assert from 'node:assert/strict';
import { createServer } from 'node:http';
import { once } from 'node:events';
import { WebClient } from '@slack/web-api';
import { Message, SectionBlock, message, Confirmation, RichTextEmoji, Button, LayoutBlock, LayoutRow, LayoutItem } from '@nicklambourne/slackblocks';

const payload = Message().channel('C_TEST').text('SDK fallback').blocks(SectionBlock().text('SDK body').accessory(Button().text('Ask').agentPrompt('Ask Slackbot').agentPromptDisplay('Summary'))).build();
assert.equal(Confirmation().title('Confirm').text('Proceed?').confirm('Yes').deny('No').style('danger').build().style, 'danger');
assert.equal(RichTextEmoji().name('wave').skinTone(6).build().skin_tone, 6);
if (false) {
  // @ts-expect-error Required destination accepts strings, not numbers.
  Message().channel(42);
  // @ts-expect-error Unknown confirmation styles must fail compilation.
  Confirmation().style('invalid');
}
const received: URLSearchParams[] = [];
const server = createServer(async (request, response) => {
  const chunks: Buffer[] = [];
  for await (const chunk of request) chunks.push(Buffer.from(chunk));
  assert.equal(request.url, '/chat.postMessage');
  received.push(new URLSearchParams(Buffer.concat(chunks).toString()));
  response.writeHead(200, { 'content-type': 'application/json' });
  response.end(JSON.stringify({ ok: true, channel: 'C_TEST', ts: '123.456' }));
});
server.listen(0, '127.0.0.1');
await once(server, 'listening');
try {
  const address = server.address();
  assert.ok(address && typeof address !== 'string');
  const client = new WebClient('synthetic-local-test', {
    slackApiUrl: `http://127.0.0.1:${address.port}/`, retryConfig: { retries: 0 },
  });
  await client.chat.postMessage(payload);
  await client.chat.postMessage(message({ channel: 'C_TEST', text: 'Legacy factory' }));
  const layout = LayoutBlock().label('SDK layout').rows(LayoutRow().items(LayoutItem().block(SectionBlock().text('Native child')).width('wide').horizontalAlignment('end'))).fallback(SectionBlock().text('Fallback')).build();
  const layoutPayload = Message().channel('C_TEST').text('Layout fallback').blocks(layout).build();
  await client.chat.postMessage(layoutPayload);
  assert.equal(received.length, 3);
  assert.deepEqual(JSON.parse(received[2]?.get('blocks') ?? 'null'), [layout]);
  const blocks = JSON.parse(received[0]?.get('blocks') ?? 'null');
  assert.equal(blocks[0].accessory.agent_prompt, 'Ask Slackbot');
  assert.equal(blocks[0].accessory.agent_prompt_display, 'Summary');
  assert.equal(received[0]?.get('channel'), 'C_TEST');
  assert.equal(received[0]?.get('text'), 'SDK fallback');
  assert.deepEqual(JSON.parse(received[0]?.get('blocks') ?? 'null'), payload.blocks);
  console.log('Installed package: strict SDK types and three loopback HTTP requests passed');
} finally {
  server.close();
  await once(server, 'close');
}
