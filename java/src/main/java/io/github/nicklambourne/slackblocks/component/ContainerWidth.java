package io.github.nicklambourne.slackblocks.component;

/** Widths supported by Slack container blocks. */
public enum ContainerWidth {
  /** A compact container. */
  NARROW("narrow"),
  /** Slack's standard container width. */
  STANDARD("standard"),
  /** A wide container. */
  WIDE("wide"),
  /** The full available width. */
  FULL("full");

  private final String wireValue;

  ContainerWidth(String wireValue) {
    this.wireValue = wireValue;
  }

  /**
   * Returns Slack's JSON value.
   *
   * @return lowercase wire value
   */
  public String wireValue() {
    return wireValue;
  }
}
