package slackblocks

// validateEnumField distinguishes a missing optional field from every supplied
// value, including an empty string. Requiredness is checked by the caller.
func validateEnumField(object Object, field, path string, choices ...string) error {
	raw, present := object[field]
	if !present {
		return nil
	}
	if value, ok := raw.(string); ok {
		for _, choice := range choices {
			if value == choice {
				return nil
			}
		}
	}
	return validationError(TypeMismatch, child(path, field), "expected one of %v", choices)
}
