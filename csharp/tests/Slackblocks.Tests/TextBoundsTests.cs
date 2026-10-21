using Slackblocks.Elements;
namespace Slackblocks.Tests;

public sealed class TextBoundsTests
{
    [Fact]
    public void ValidatesOnlyWhenBothBoundsAreSupplied()
    {
        foreach (var pair in new (int? Min, int? Max)[] { (null, null), (20, null), (null, 10), (0, 1), (10, 10), (3000, 3000) })
        {
            var value = new PlainTextInputElement(minLength: pair.Min, maxLength: pair.Max);
            Assert.Equal(pair.Min, value.MinLength);
            Assert.Equal(pair.Max, value.MaxLength);
        }
        var error = Assert.Throws<ValidationException>(() => new PlainTextInputElement(minLength: 20, maxLength: 10));
        Assert.Equal(ErrorCategory.OutOfRange, error.Category);
        Assert.Equal("PlainTextInputElement.min_length", error.Path);
    }
    [Fact]
    public void StructuralControlsRemainAccepted()
    {
        Assert.Equal(0, new ExternalSelectElement(minQueryLength: 0).MinQueryLength);
        Assert.Equal(0, new ExternalMultiSelectElement(minQueryLength: 0).MinQueryLength);
        foreach (var date in new[] { "2028-02-29", "2028-02-30" }) Assert.Equal(date, new DatePickerElement(initialDate: date).InitialDate);
        foreach (var time in new[] { "00:00", "23:59" }) Assert.Equal(time, new TimePickerElement(initialTime: time).InitialTime);
        Assert.Equal("", new ImageElement(imageUrl: "https://example.com/image.png", altText: "").AltText);
    }
}
