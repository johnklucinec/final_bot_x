const { Rettiwt } = require('rettiwt-api');

// Create a new Rettiwt instance without an API key
const rettiwt = new Rettiwt();

// Fetch the tweet timeline of the user whose ID is '3002817576'
rettiwt.user.timeline('3002817576')

.then(timeline => {

  if (timeline.list.length > 0) {
    // The most recent tweet is the first entry in the timeline array
    const mostRecentTweet = timeline.list[0];
    // Construct the URL of the most recent tweet
    const tweetUrl = `https://fxtwitter.com/finalmouse/status/${mostRecentTweet.id}`;
    console.log(tweetUrl);
  } else {
    console.log('No tweets found');
  }
})
.catch(error => {
 console.error(error);
});




// remember to run npm install --save rettiwt-api