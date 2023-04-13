'use strict'

module.exports = {
  'extends': '@answerbook/release-config-logdna'
, 'branches': ["main"]
, "plugins": [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    "@semantic-release/changelog",
    ["@semantic-release/exec", {
      "prepareCmd": "cargo set-version ${nextRelease.version} && cargo package --allow-dirty --target-dir dist; sleep 2"
    }],
    "@semantic-release/github",
    ["@semantic-release/git", {
      "assets": [
        "CHANGELOG.md",
        "package.json",
        "Cargo*",
        "dist/*"
      ],
      "message": 'release: ${year}-${month}-${day}, Version <%= nextRelease.version %> [skip ci]'
    }]
  ]
}
