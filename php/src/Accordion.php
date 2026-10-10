<?php

declare(strict_types=1);

namespace Slackblocks;

/** Expands validated collapsible containers into ordinary Slack blocks. */
final class Accordion
{
    /** @param list<ContainerBlock> $sections Ordered, nonempty collapsible sections.
     * @return list<ContainerBlock> Blocks to spread into a modal or Home tab.
     */
    public static function create(array $sections): array
    {
        if ($sections === []) {
            throw new ValidationError(ErrorCategory::MissingRequired, 'Accordion.sections', 'expected at least one section');
        }
        if (!array_is_list($sections)) {
            throw new ValidationError(ErrorCategory::TypeMismatch, 'Accordion.sections', 'expected a list');
        }
        foreach ($sections as $i => $section) {
            if (!$section instanceof ContainerBlock || $section->isCollapsible !== true) {
                throw new ValidationError(ErrorCategory::TypeMismatch, 'Accordion.sections[' . $i . ']', 'expected a collapsible container');
            }
        }
        return $sections;
    }
}
