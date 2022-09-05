<script type='ts'>
  export let post;

  let processedText = post.text.replace(/https:\/\/t.co\/\w+(\s|$)/g, "");
</script>

<div class="post">
  <span class="text">{processedText}</span>
  <span class="debug">{JSON.stringify(post)}</span>
</div>

<style>
  .post {
    white-space: pre-line;
    width: 100%;
    border-top: 1px solid #ccc;
    padding: 0.2rem 0;
    min-height: 1rem;
  }

  .debug {
    font-size: 0.6rem;
    color: #666;
  }
</style>
