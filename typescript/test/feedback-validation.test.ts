import { describe, expect, it } from 'vitest';
import { FeedbackButton, FeedbackButtons, LengthError, feedbackButton } from '../src/index.js';

describe('standalone feedback choices', () => {
  it.each([['text', 75], ['value', 2000], ['accessibilityLabel', 75]] as const)('%s has the same standalone and parent limit', (field, maximum) => {
    const choice = (size: number) => FeedbackButton().text('Yes').value('yes')[field]('😀'.repeat(size));
    expect(() => choice(maximum).build()).not.toThrow();
    expect(() => choice(maximum + 1).build()).toThrow(LengthError);
    expect(() => FeedbackButtons().positiveButton(choice(maximum + 1)).negativeButton(FeedbackButton().text('No').value('no')).build()).toThrow(LengthError);
    // Validation-disabled partial objects are still rejected by their validating parent.
    const partial = feedbackButton({ text: 'Yes', value: 'yes', [field]: '😀'.repeat(maximum + 1) }, { validate: false });
    expect(() => FeedbackButtons().positiveButton(partial).negativeButton(FeedbackButton().text('No').value('no')).build()).toThrow(LengthError);
  });
});
