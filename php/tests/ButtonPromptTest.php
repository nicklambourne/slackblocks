<?php

declare(strict_types=1);

namespace Slackblocks\Tests;

use PHPUnit\Framework\TestCase;
use Slackblocks\{ButtonElement, ErrorCategory, JsonObject, ValidationError};

final class ButtonPromptTest extends TestCase
{
    public function testNativeFieldsAndLegacyPositionalExtensions(): void
    {
        $legacy = new ButtonElement('Ask', 'ask', null, null, null, null, null, new JsonObject(['custom' => 'kept']));
        self::assertSame('kept', json_decode($legacy->toJson(), true)['custom']);
        $button = new ButtonElement('Ask', agentPrompt: 'Ask Slackbot', agentPromptDisplay: str_repeat('x', 80));
        self::assertSame('Ask Slackbot', $button->agentPrompt);
        self::assertEquals($button, ButtonElement::fromJson($button->toJson()));
        self::assertSame('Changed', $button->with(agentPrompt: 'Changed')->agentPrompt);
        self::assertSame('Ask Slackbot', $button->agentPrompt);
        self::assertNull((new ButtonElement('Ask', agentPrompt: 'Ask Slackbot'))->agentPromptDisplay);
    }

    public function testCheckedIngressAndCopiesRetainValidation(): void
    {
        foreach (['agent_prompt', 'agent_prompt_display'] as $field) {
            try {
                ButtonElement::fromJson(json_encode(['type' => 'button', 'text' => ['type' => 'plain_text', 'text' => 'Ask'], $field => 1], JSON_THROW_ON_ERROR));
                self::fail('accepted a non-string prompt field');
            } catch (ValidationError $error) {
                self::assertSame(ErrorCategory::TypeMismatch, $error->category);
            }
        }
        $this->expectException(ValidationError::class);
        (new ButtonElement('Ask', agentPrompt: '', agentPromptDisplay: ''))->with(agentPrompt: null);
    }
}
