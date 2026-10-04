use crate::{
    Block, ContainerBlock, ContainerBlockBuilder, ContainerWidth, ErrorCategory, PlainTextInput,
    ValidationError,
};

/// A validated, collapsible section of an [`Accordion`].
#[derive(Clone, Debug, PartialEq)]
pub struct AccordionSection(ContainerBlock);

/// Consuming builder for an accordion section.
#[derive(Clone, Debug)]
pub struct AccordionSectionBuilder(ContainerBlockBuilder);
impl AccordionSection {
    /// Starts a collapsed section with standard width and the supplied plain-text title.
    pub fn builder(title: impl Into<PlainTextInput>) -> AccordionSectionBuilder {
        AccordionSectionBuilder(
            ContainerBlock::builder()
                .title(title)
                .width(ContainerWidth::Standard)
                .is_collapsible(true)
                .default_collapsed(true)
                .has_header_divider(false),
        )
    }
    /// Borrows the validated container representation.
    pub fn as_container(&self) -> &ContainerBlock {
        &self.0
    }
    /// Moves the section into its container block representation.
    pub fn into_container(self) -> ContainerBlock {
        self.0
    }
}
impl AccordionSectionBuilder {
    /// Adds a subtitle with field-specific text coercion.
    pub fn subtitle(mut self, subtitle: impl Into<crate::TextInput>) -> Self {
        self.0 = self.0.subtitle(subtitle);
        self
    }
    /// Adds a validated image icon.
    pub fn icon(mut self, icon: crate::ImageElement) -> Self {
        self.0 = self.0.icon(icon);
        self
    }
    /// Selects the section width.
    pub fn width(mut self, width: ContainerWidth) -> Self {
        self.0 = self.0.width(width);
        self
    }
    /// Sets the header-divider flag. Slack rejects `true` on collapsible sections.
    pub fn has_header_divider(mut self, divider: bool) -> Self {
        self.0 = self.0.has_header_divider(divider);
        self
    }

    /// Replaces the section's child blocks, retaining their order.
    pub fn blocks<I, T>(mut self, blocks: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Block>,
    {
        self.0 = self.0.child_blocks(blocks);
        self
    }
    /// Appends a child block.
    pub fn block(mut self, block: impl Into<Block>) -> Self {
        self.0 = self.0.child_block(block);
        self
    }
    /// Controls whether the section initially appears expanded.
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.0 = self.0.default_collapsed(!expanded);
        self
    }
    /// Sets the section's block identifier.
    pub fn block_id(mut self, id: impl Into<String>) -> Self {
        self.0 = self.0.block_id(id);
        self
    }
    /// Builds a validated collapsible container.
    ///
    /// # Errors
    /// Returns an error for invalid titles, missing/too many blocks or invalid identifiers.
    pub fn build(self) -> Result<AccordionSection, ValidationError> {
        self.0.build().map(AccordionSection)
    }
}
impl From<AccordionSection> for Block {
    fn from(section: AccordionSection) -> Self {
        section.0.into()
    }
}

/// A nonempty ordered collection of collapsible sections.
///
/// Rendering produces ordinary blocks; the receiving message/view builder checks
/// its own surface and total block-count constraints after expansion.
#[derive(Clone, Debug, PartialEq)]
pub struct Accordion {
    sections: Vec<AccordionSection>,
}
impl Accordion {
    /// Collects validated sections in display order.
    ///
    /// # Errors
    /// Returns `missing-required` when no section is supplied.
    pub fn new(
        sections: impl IntoIterator<Item = AccordionSection>,
    ) -> Result<Self, ValidationError> {
        let sections: Vec<_> = sections.into_iter().collect();
        if sections.is_empty() {
            return Err(ValidationError::new(
                ErrorCategory::MissingRequired,
                "Accordion.sections",
                "expected at least one section",
            ));
        }
        Ok(Self { sections })
    }
    /// Borrows the ordered sections.
    pub fn sections(&self) -> &[AccordionSection] {
        &self.sections
    }
    /// Renders owned blocks while retaining this accordion for reuse.
    pub fn render(&self) -> Vec<Block> {
        self.sections.iter().cloned().map(Into::into).collect()
    }
}
impl IntoIterator for Accordion {
    type Item = Block;
    type IntoIter = std::vec::IntoIter<Block>;
    fn into_iter(self) -> Self::IntoIter {
        self.sections
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>()
            .into_iter()
    }
}

/// A validated page of blocks and its navigation controls.
///
/// Rendered blocks are ordinary values. Receiving message/view builders validate
/// surface and aggregate constraints after pagination has expanded the controls.
#[derive(Clone, Debug, PartialEq)]
pub struct Paginator {
    blocks: Vec<Block>,
    page: usize,
    page_count: usize,
}
/// Consuming configuration for a [`Paginator`].
#[derive(Clone, Debug)]
pub struct PaginatorBuilder {
    blocks: Vec<Block>,
    action_id_prefix: String,
    page: usize,
    page_size: usize,
    previous_text: String,
    next_text: String,
    show_page_indicator: bool,
    block_id: Option<String>,
}
impl Paginator {
    /// Starts at page one with five content blocks per page.
    pub fn builder<I, T>(action_id_prefix: impl Into<String>, blocks: I) -> PaginatorBuilder
    where
        I: IntoIterator<Item = T>,
        T: Into<Block>,
    {
        PaginatorBuilder {
            blocks: blocks.into_iter().map(Into::into).collect(),
            action_id_prefix: action_id_prefix.into(),
            page: 1,
            page_size: 5,
            previous_text: "Previous".into(),
            next_text: "Next".into(),
            show_page_indicator: true,
            block_id: None,
        }
    }
    /// Returns the current one-based page number.
    pub const fn page(&self) -> usize {
        self.page
    }
    /// Returns the total number of pages.
    pub const fn page_count(&self) -> usize {
        self.page_count
    }
    /// Borrows this page's content and controls.
    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }
    /// Clones this page's content and controls for reuse.
    pub fn render(&self) -> Vec<Block> {
        self.blocks.clone()
    }
}
impl PaginatorBuilder {
    /// Selects a one-based page number.
    pub fn page(mut self, page: usize) -> Self {
        self.page = page;
        self
    }
    /// Sets the positive number of content blocks on each page.
    pub fn page_size(mut self, size: usize) -> Self {
        self.page_size = size;
        self
    }
    /// Sets the previous button's plain-text label.
    pub fn previous_text(mut self, text: impl Into<String>) -> Self {
        self.previous_text = text.into();
        self
    }
    /// Sets the next button's plain-text label.
    pub fn next_text(mut self, text: impl Into<String>) -> Self {
        self.next_text = text.into();
        self
    }
    /// Controls the `Page N of M` context block.
    pub fn show_page_indicator(mut self, show: bool) -> Self {
        self.show_page_indicator = show;
        self
    }
    /// Sets the navigation actions block identifier.
    pub fn block_id(mut self, id: impl Into<String>) -> Self {
        self.block_id = Some(id.into());
        self
    }
    /// Builds this page, omitting controls when only one page exists.
    ///
    /// # Errors
    /// Rejects empty content/prefixes, zero page sizes, out-of-range pages, and
    /// invalid navigation labels or action identifiers.
    pub fn build(self) -> Result<Paginator, ValidationError> {
        use crate::{ActionsBlock, ButtonElement, ContextBlock, MarkdownText};
        let fail = |category, path, message| ValidationError::new(category, path, message);
        if self.blocks.is_empty() {
            return Err(fail(
                ErrorCategory::MissingRequired,
                "Paginator.blocks",
                "expected at least one block",
            ));
        }
        if self.action_id_prefix.is_empty() {
            return Err(fail(
                ErrorCategory::MissingRequired,
                "Paginator.action_id_prefix",
                "expected a nonempty prefix",
            ));
        }
        if self.page_size == 0 {
            return Err(fail(
                ErrorCategory::OutOfRange,
                "Paginator.page_size",
                "expected a positive page size",
            ));
        }
        let page_count = self.blocks.len().div_ceil(self.page_size);
        if self.page == 0 || self.page > page_count {
            return Err(fail(
                ErrorCategory::OutOfRange,
                "Paginator.page",
                "page is outside the available pages",
            ));
        }
        let mut blocks: Vec<_> = self
            .blocks
            .into_iter()
            .skip((self.page - 1) * self.page_size)
            .take(self.page_size)
            .collect();
        if page_count > 1 {
            let mut controls = Vec::new();
            if self.page > 1 {
                controls.push(
                    ButtonElement::builder()
                        .text(self.previous_text)
                        .action_id(format!("{}.previous", self.action_id_prefix))
                        .value((self.page - 1).to_string())
                        .build()?,
                );
            }
            if self.page < page_count {
                controls.push(
                    ButtonElement::builder()
                        .text(self.next_text)
                        .action_id(format!("{}.next", self.action_id_prefix))
                        .value((self.page + 1).to_string())
                        .build()?,
                );
            }
            if self.show_page_indicator {
                blocks.push(
                    ContextBlock::builder()
                        .element(MarkdownText::new(format!(
                            "Page {} of {page_count}",
                            self.page
                        ))?)
                        .build()?
                        .into(),
                );
            }
            let mut actions = ActionsBlock::builder().elements(controls);
            if let Some(id) = self.block_id {
                actions = actions.block_id(id);
            }
            blocks.push(actions.build()?.into());
        }
        Ok(Paginator {
            blocks,
            page: self.page,
            page_count,
        })
    }
}
impl IntoIterator for Paginator {
    type Item = Block;
    type IntoIter = std::vec::IntoIter<Block>;
    fn into_iter(self) -> Self::IntoIter {
        self.blocks.into_iter()
    }
}
