defmodule Slackblocks.ValidationError do
  @moduledoc "A categorized validation failure with wire keys and zero-based indexes in `path`."
  defexception [:category, :path, :message]

  @type category ::
          :length_exceeded
          | :out_of_range
          | :mutually_exclusive
          | :type_mismatch
          | :missing_required
          | :invalid_usage
  @type t :: %__MODULE__{
          category: category(),
          path: [binary() | non_neg_integer()],
          message: binary()
        }
end
