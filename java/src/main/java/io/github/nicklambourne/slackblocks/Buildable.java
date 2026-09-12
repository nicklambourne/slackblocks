package io.github.nicklambourne.slackblocks;

/**
 * A concrete fluent builder that materializes one immutable Slack value.
 *
 * @param <T> immutable Slack value produced by this builder
 */
@FunctionalInterface
public interface Buildable<T extends SlackObject> {
  /**
   * Builds and validates an immutable value.
   *
   * @return a defensive snapshot of the configured value
   */
  T build();
}
