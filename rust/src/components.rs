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
