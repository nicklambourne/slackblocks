# Views

**Views** are app-customised UI surfaces inside Slack — the contents of a modal dialog or the App Home tab.

- `ModalView` — content of a modal opened with `views.open` / `views.update` / `views.push`.
- `HomeTabView` — content of the App Home tab, published with `views.publish`.

Pass `view.to_dict()` as the `view` argument to the Slack SDK's view methods.

Slack reference: <https://api.slack.com/reference/surfaces/views>

::: views
    options:
        filters: ["!^View$", "!^ViewType"]
        show_bases: false
