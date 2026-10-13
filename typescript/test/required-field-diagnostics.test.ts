import { describe, expect, it } from 'vitest';
import { AxisConfig, FeedbackButton, PlainText, Markdown, Message, Confirmation, InputParameter, RawNumber, RawText, Trigger, Workflow, FileBlock, IconButton, ChartSegment, DataPoint, DataSeries, HomeTab, Modal, Option, OptionGroup, MissingRequiredError, TypeMismatchError } from '../src/index.js';

const missing: [string, () => unknown][] = [
  ['chartSegment.value', () => ChartSegment().label('A').build()],
  ['dataPoint.value', () => DataPoint().label('A').build()],
  ['feedbackButton.text', () => FeedbackButton().value('yes').build()],
  ['feedbackButton.value', () => FeedbackButton().text('Yes').build()],
  ['text', () => PlainText().build()],
  ['text', () => Markdown().build()],
  ['rawNumber.value', () => RawNumber().text('One').build()],
  ['message.channel', () => Message().text('Body').build()],
  ['confirmation.title', () => Confirmation().text('Body').confirm('Yes').deny('No').build()],
  ['confirmation.text', () => Confirmation().title('Title').confirm('Yes').deny('No').build()],
  ['confirmation.confirm', () => Confirmation().title('Title').text('Body').deny('No').build()],
  ['confirmation.deny', () => Confirmation().title('Title').text('Body').confirm('Yes').build()],
  ['inputParameter.name', () => InputParameter().value('v').build()],
  ['inputParameter.value', () => InputParameter().name('n').build()],
  ['option.text', () => Option().value('v').build()],
  ['optionGroup.label', () => OptionGroup().options(Option().text('T').value('v')).build()],
  ['rawNumber.text', () => RawNumber().value(1).build()],
  ['rawText.text', () => RawText().build()],
  ['trigger.url', () => Trigger().build()],
  ['workflow.trigger', () => Workflow().build()],
  ['dataPoint.label', () => DataPoint().value(1).build()],
  ['dataSeries.name', () => DataSeries().data(DataPoint().label('A').value(1)).build()],
  ['dataSeries.data', () => DataSeries().name('Series').build()],
  ['axisConfig.categories', () => AxisConfig().build()],
  ['option.value', () => Option().text('Choice').build()],
  ['chartSegment.label', () => ChartSegment().value(1).build()],
  ['optionGroup.options', () => OptionGroup().label('Group').build()],
  ['homeTab.blocks', () => HomeTab().build()],
  ['modal.blocks', () => Modal().title('Title').build()],
];
describe('required factory field diagnostics', () => {
  it('retains documented defaults and aliases', () => {
    expect(FileBlock().externalId('remote-file').build().source).toBe('remote');
    expect(IconButton().text('Review').build().icon).toBeDefined();
  });
  it.each(missing)('%s produces a path-aware missing-field error', (path, build) => {
    expect(build).toThrow(MissingRequiredError);
    expect(build).toThrow(path);
  });
  it('distinguishes malformed values from omissions before iteration', () => {
    // @ts-expect-error Deliberate untyped caller passing a number.
    expect(() => DataPoint().label(3).value(1).build()).toThrow(TypeMismatchError);
    // @ts-expect-error Deliberate untyped caller passing null instead of a string list.
    expect(() => AxisConfig().categories(null).build()).toThrow(TypeMismatchError);
    // @ts-expect-error Deliberate untyped caller passing null instead of blocks.
    expect(() => HomeTab().blocks(null).build()).toThrow(TypeMismatchError);
  });
  it('distinguishes invalid text arguments from absent text arguments', () => {
    // @ts-expect-error Deliberate untyped text input.
    expect(() => Confirmation().title(3).text('Body').confirm('Yes').deny('No').build()).toThrow(TypeMismatchError);
    // @ts-expect-error Deliberate untyped text input.
    expect(() => Option().text(3).value('v').build()).toThrow(TypeMismatchError);
    // @ts-expect-error Deliberate untyped text input.
    expect(() => OptionGroup().label(3).options(Option().text('T').value('v')).build()).toThrow(TypeMismatchError);
  });
  it('keeps valid fluent collection accumulation and astral labels', () => {
    const point = DataPoint().label('😀').value(1);
    expect(DataSeries().name('Series').data(point).build().data).toEqual([{ label: '😀', value: 1 }]);
    expect(AxisConfig().categories('😀').build().categories).toEqual(['😀']);
  });
});
