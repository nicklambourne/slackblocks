using System;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using Slackblocks.Blocks;
using Slackblocks.Elements;
using Slackblocks.Objects;

namespace Slackblocks.Components;

/// <summary>Selects one page of blocks and adds standard navigation controls.</summary>
/// <remarks>
/// The result is an ordinary list of blocks. When there is more than one page it ends with an
/// optional <c>Page N of M</c> context block and an actions block holding previous and next
/// buttons. Button action identifiers are <c>{actionIdPrefix}.previous</c> and
/// <c>{actionIdPrefix}.next</c>, and their values carry the one-based page number to show.
/// </remarks>
public static class Paginator
{
    /// <summary>Builds one page of <paramref name="blocks"/> with navigation.</summary>
    /// <param name="actionIdPrefix">Prefix for the navigation buttons' action identifiers.</param>
    /// <param name="blocks">Every block across all pages, in display order.</param>
    /// <param name="page">The one-based page to show.</param>
    /// <param name="pageSize">How many blocks each page shows.</param>
    /// <param name="previousText">Label of the previous-page button.</param>
    /// <param name="nextText">Label of the next-page button.</param>
    /// <param name="showPageIndicator">Whether to add a <c>Page N of M</c> context block.</param>
    /// <param name="blockId">Block identifier for the navigation actions block.</param>
    /// <returns>The blocks for the requested page, followed by the navigation blocks when needed.</returns>
    /// <exception cref="ValidationException">
    /// There are no blocks, the prefix is empty, or the page or page size is out of range.
    /// </exception>
    public static IReadOnlyList<IBlock> Create(
        string actionIdPrefix,
        IEnumerable<IBlock> blocks,
        int page = 1,
        int pageSize = 5,
        string previousText = "Previous",
        string nextText = "Next",
        bool showPageIndicator = true,
        string? blockId = null)
    {
        ArgumentNullException.ThrowIfNull(blocks);
        var all = blocks.ToList();
        if (all.Count == 0)
        {
            throw new ValidationException(ErrorCategory.MissingRequired, "Paginator.blocks", "expected at least one block");
        }

        if (string.IsNullOrEmpty(actionIdPrefix))
        {
            throw new ValidationException(ErrorCategory.MissingRequired, "Paginator.actionIdPrefix", "expected a non-empty prefix");
        }

        Positive("Paginator.page", page);
        Positive("Paginator.pageSize", pageSize);
        var pageCount = (all.Count + pageSize - 1) / pageSize;
        if (page > pageCount)
        {
            throw new ValidationException(
                ErrorCategory.OutOfRange,
                "Paginator.page",
                "expected a value between 1 and " + pageCount.ToString(CultureInfo.InvariantCulture));
        }

        var start = (page - 1) * pageSize;
        var result = all.Skip(start).Take(pageSize).ToList();
        if (pageCount == 1)
        {
            return result.AsReadOnly();
        }

        var controls = new List<ButtonElement>();
        if (page > 1)
        {
            controls.Add(new ButtonElement(
                previousText,
                actionIdPrefix + ".previous",
                value: (page - 1).ToString(CultureInfo.InvariantCulture)));
        }

        if (page < pageCount)
        {
            controls.Add(new ButtonElement(
                nextText,
                actionIdPrefix + ".next",
                value: (page + 1).ToString(CultureInfo.InvariantCulture)));
        }

        if (showPageIndicator)
        {
            result.Add(new ContextBlock([
                new MarkdownText(string.Create(CultureInfo.InvariantCulture, $"Page {page} of {pageCount}")),
            ]));
        }

        result.Add(new ActionsBlock(controls, blockId: blockId));
        return result.AsReadOnly();
    }

    private static void Positive(string path, int value)
    {
        if (value < 1)
        {
            throw new ValidationException(ErrorCategory.OutOfRange, path, "expected a positive integer");
        }
    }
}
