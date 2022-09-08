<script type='ts'>
  // tweet itself
  export let post;
  // users[user_id] = user information;
  export let users = {};
  // refTweets[tweet_id] = tweet content;
  export let refTweets = {};

  let json = JSON.stringify(post, null, "  ");

  let opened = false;
  let fullRawText = post.text;
  $: fullRawText = retweet ? retweetSrc.text : post.text;
  let text = "";
  $: {
    text = fullRawText
      .replace(/https:\/\/t.co\/\w+(\s|$)/g, "")
      .replace(/^RT @([^:])+: /g, "");
  }
  let author = users[post.author_id];
  let links = [];
  if (post.entities && post.entities.urls) {
    links = post.entities.urls;
  }

  let retweet, retweetSrc, retweetSrcAuthor;
  let quote = false;
  $: retweet = retweetSrc !== undefined;
  if (post.referenced_tweets) {
    for (const ref of post.referenced_tweets) {
      if (ref.type === "retweeted") {
        retweetSrc = refTweets[ref.id];
        retweetSrcAuthor = users[retweetSrc.author_id];
        break;
      }
    }
  }

  function toggle() {
    opened = !opened;
  }
</script>

<div class="post" on:click={toggle}>
  <span class="text">
    {text}<br>
    {#each links as link}
      <a class="link" href={link.url}>{link.title || link.url}</a>
    {/each}
  </span>
  <div class="detail" class:open={opened}>
    {#if author}
      {#if retweet}
        <span class="author">@{retweetSrcAuthor.username} (@{author.username} retweeted)</span>
      {:else}
        <span class="author">@{author.username}</span>
      {/if}
    {/if}
    <span class="debug">{json}</span>
  </div>
</div>

<style>
  .post {
    white-space: pre-line;
    width: 100%;
    border-top: 1px solid #ddd;
    padding: 0.2rem 0;
    min-height: 1rem;
  }

  .debug {
    display: block;
    font-size: 0.6rem;
    font-family: monospace;
    color: #666;
    white-space: pre-wrap;
  }

  .detail {
    display: none;
  }

  .detail.open {
    display: block;
  }

  .author {
    font-size: 0.8rem;
  }

  .link {
    color: #444;
    margin-right: 0.3rem;
  }
</style>
