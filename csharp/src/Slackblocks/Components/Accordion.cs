using System;
using System.Collections.Generic;
using System.Linq;
using Slackblocks.Blocks;
using Slackblocks.Elements;
using Slackblocks.Objects;

namespace Slackblocks.Components;

/// <summary>A stack of independently collapsible sections, each a Slack container block.</summary>
/// <remarks>
/// Build the sections with <see cref="AccordionSection.Create"/>. The result is an ordinary list of
/// blocks that can be combined with other blocks, for example
/// <c>blocks: [header, ..Accordion.Create(sections)]</c>.
/// </remarks>
public static class Accordion
{
    /// <summary>Combines accordion sections into a list of blocks.</summary>
    /// <param name="sections">Sections created by <see cref="AccordionSection.Create"/>, in display order.</param>
    /// <returns>The sections as blocks.</returns>
    /// <exception cref="ValidationException">
    /// There are no sections, or a section is not a collapsible container.
    /// </exception>
    public static IReadOnlyList<IBlock> Create(IEnumerable<ContainerBlock> sections)
    {
        ArgumentNullException.ThrowIfNull(sections);
        var all = sections.ToList();
        if (all.Count == 0)
        {
            throw new ValidationException(ErrorCategory.MissingRequired, "Accordion.sections", "expected at least one section");
        }

        for (var index = 0; index < all.Count; index++)
        {
            if (all[index] is not { IsCollapsible: true })
            {
                throw new ValidationException(
                    ErrorCategory.TypeMismatch,
                    $"Accordion.sections[{index}]",
                    "expected an AccordionSection");
            }
        }

        return all.Cast<IBlock>().ToList().AsReadOnly();
    }
}

/// <summary>Creates one collapsible accordion section.</summary>
public static class AccordionSection
{
    /// <summary>Creates a collapsible container block for use in an <see cref="Accordion"/>.</summary>
    /// <param name="title">Heading shown whether the section is expanded or collapsed.</param>
    /// <param name="blocks">Blocks revealed when the section is expanded.</param>
    /// <param name="subtitle">Supporting text below the heading. A string is sent as <c>mrkdwn</c>.</param>
    /// <param name="icon">Image shown beside the heading.</param>
    /// <param name="expanded">Whether the section starts expanded.</param>
    /// <param name="width">Container width.</param>
    /// <param name="hasHeaderDivider">Whether a divider separates the heading from the content.</param>
    /// <param name="blockId">Block identifier for the container.</param>
    /// <returns>A validated collapsible <see cref="ContainerBlock"/>.</returns>
    /// <exception cref="ValidationException">The section breaks a Block Kit rule.</exception>
    public static ContainerBlock Create(
        PlainText title,
        IEnumerable<IBlock> blocks,
        Text? subtitle = null,
        ImageElement? icon = null,
        bool expanded = false,
        ContainerWidth width = ContainerWidth.Standard,
        bool hasHeaderDivider = false,
        string? blockId = null) =>
        new(
            childBlocks: blocks,
            title: title,
            subtitle: subtitle,
            width: width,
            icon: icon,
            isCollapsible: true,
            defaultCollapsed: !expanded,
            hasHeaderDivider: hasHeaderDivider,
            blockId: blockId);
}
