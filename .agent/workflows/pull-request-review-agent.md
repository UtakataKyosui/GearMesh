---
description: PRについたレビューに応答し、修正を行ってPushする
---

カレントブランチで作成されたPRに対するコードレビューを
ghまたはMCPで取得し、その内容に沿って修正を行い、
Linter.Formatter,型チェックなどを行ってからCommitしてください。

ghを使うケースだと以下のようなコマンドになります。

```shell
# ~/.bashrc や ~/.zshrc に追加
owner=$(gh repo view --json owner --jq .owner.login)
repo=$(gh repo view --json name --jq .name)
pr=$(gh pr view --json number --jq .number)
  
gh api graphql -f query="
  query {
    repository(owner: \"$owner\", name: \"$repo\") {
      pullRequest(number: $pr) {
        reviewThreads(first: 50) {
          nodes {
            id
            isResolved
            comments(first: 1) {
              nodes {
                body
              }
            }
          }
        }
      }
    }
  }" --jq '.data.repository.pullRequest.reviewThreads.nodes[] | select(.isResolved == false)'
```

レビューコメントには、そのレビューの概要文章と、
そのレビューで特に修正すべきコードの変更対象と変更提案が載っているものがあります。
その両方を取得して修正してください。

Pre-Commitでそれらのチェックが行われる場合は工程をスキップしても構いません。
また、その対応ができた時点で、当該レビューを「Resolve conversation」してください。

Resolve conversationのため、ghでGitHub GraphQL APIを使用し、
resolveReviewThreadに対して、isResolveをTrueにすることで、「Resolve conversation」に相当する操作が可能です。

ghを使用していて、権限が足りなくて操作が完了しなかった場合、
`gh auth refresh`を`-s repo,read:org,write:discussion`という引数を与えて
実行させることを推奨してください。
