package io.github.nicklambourne.slackblocks.component;

import io.github.nicklambourne.slackblocks.ErrorCategory;
import io.github.nicklambourne.slackblocks.ValidationException;
import io.github.nicklambourne.slackblocks.block.Block;
import io.github.nicklambourne.slackblocks.block.ContainerBlock;
import java.util.ArrayList;
import java.util.List;

/**
 * Assembles independently collapsible sections. Slack handles expanding and collapsing, so no
 * interaction handler is needed. Pass the result to the Slack SDK with {@code List.copyOf}, which
 * widens it to {@code List<LayoutBlock>}.
 */
public final class Accordion {
  private Accordion() {}

  /**
   * Starts a concrete fluent accordion builder.
   *
   * @return a new builder
   */
  public static Builder builder() {
    return new Builder();
  }

  /** Concrete fluent accordion builder. */
  public static final class Builder {
    private final List<ContainerBlock> sections = new ArrayList<>();

    private Builder() {}

    /**
     * Appends built accordion sections in display order.
     *
     * @param values sections created with {@link AccordionSection}
     * @return this builder
     */
    public Builder sections(ContainerBlock... values) {
      sections.addAll(List.of(values));
      return this;
    }

    /**
     * Builds ordinary Slack container blocks ready for an SDK request.
     *
     * @return immutable list of container blocks
     * @throws ValidationException if there are no sections or a section is not collapsible
     */
    public List<Block> build() {
      if (sections.isEmpty()) {
        throw new ValidationException(
            ErrorCategory.MISSING_REQUIRED, "Accordion.sections", "expected at least one section");
      }
      for (int index = 0; index < sections.size(); index++) {
        ContainerBlock section = sections.get(index);
        if (!Boolean.TRUE.equals(section.toMap().get("is_collapsible"))) {
          throw new ValidationException(
              ErrorCategory.TYPE_MISMATCH,
              "Accordion.sections[" + index + "]",
              "expected an AccordionSection");
        }
      }
      return List.copyOf(sections);
    }
  }
}
