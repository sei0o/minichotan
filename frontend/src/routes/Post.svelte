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
    const t = fullRawText
      .replace(/https:\/\/t.co\/\w+(\s|$)/g, "")
      .replace(/^RT @([^:])+: /g, "")
      .replace(/\n+$/g, "");
    const doc = new DOMParser().parseFromString(t, "text/html");
    text = doc.documentElement.textContent;
  }
  let author = users[post.author_id];
  let links = [];
  if (post.entities && post.entities.urls) {
    links = post.entities.urls;
  }

  let retweet, retweetSrc, retweetSrcAuthor;
  let quote, quoteSrc, quoteSrcAuthor;
  $: retweet = retweetSrc !== undefined;
  $: quote = quoteSrc !== undefined;
  if (post.referenced_tweets) {
    for (const ref of post.referenced_tweets) {
      if (ref.type === "retweeted") {
        retweetSrc = refTweets[ref.id];
        retweetSrcAuthor = users[retweetSrc.author_id];
        break;
      }

      if (ref.type === "quoted") {
        quoteSrc = refTweets[ref.id];
        quoteSrcAuthor = users[quoteSrc.author_id];
      }
    }
  }

  function toggle() {
    opened = !opened;
  }
</script>

<div class="post" on:click={toggle}>
  <span class="text">
    {#if quote}
      {quoteSrc.text}
    {/if}
    {text}<br>
  </span>
  <div class="links">
    {#each links as link}
      <a class="link" href={link.url}>
        {#if link.media_key}
          [Media]
        {:else}
          {link.title || link.display_url}
        {/if}
      </a>
    {/each}
  </div>
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
    font-size: 0.8rem;
  }
</style>
