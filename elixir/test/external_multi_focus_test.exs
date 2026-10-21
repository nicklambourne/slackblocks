defmodule Slackblocks.ExternalMultiFocusTest do
  use ExUnit.Case, async: true
  alias Slackblocks.Elements.{ExternalMultiSelect, ExternalSelect, UserMultiSelect}
  alias Slackblocks.ValidationError

  test "omitted or cleared focus serializes and round trips without the unsupported field" do
    value = ExternalMultiSelect.new!(action_id: "choices", placeholder: "Choose")
    refute Map.has_key?(Slackblocks.to_map!(value), "focus_on_load")
    assert Slackblocks.decode!(ExternalMultiSelect, Slackblocks.encode!(value)) == value
    assert ExternalMultiSelect.change!(value, focus_on_load: nil) == value
    assert ExternalMultiSelect.new!(focus_on_load: nil) == ExternalMultiSelect.new!()
  end

  test "explicit booleans fail construction, checked editing, ingress and forged output" do
    value = ExternalMultiSelect.new!()

    for focus <- [false, true] do
      expected = %ValidationError{category: :invalid_usage, path: ["focus_on_load"]}

      for result <- [
            ExternalMultiSelect.new(focus_on_load: focus),
            ExternalMultiSelect.change(value, focus_on_load: focus),
            Slackblocks.from_map(ExternalMultiSelect, %{
              "type" => "multi_external_select",
              "focus_on_load" => focus
            }),
            Slackblocks.to_map(%{value | focus_on_load: focus})
          ] do
        assert {:error, error} = result
        assert error.category == expected.category
        assert error.path == expected.path
      end
    end
  end

  test "supported single external and multi-user selectors preserve explicit focus" do
    for module <- [ExternalSelect, UserMultiSelect], focus <- [false, true] do
      value = module.new!(focus_on_load: focus)
      assert Slackblocks.to_map!(value)["focus_on_load"] === focus
      assert Slackblocks.decode!(module, Slackblocks.encode!(value)) == value
    end
  end
end
