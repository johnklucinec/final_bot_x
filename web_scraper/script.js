const { Rettiwt } = require('rettiwt-api');
const rettiwt = new Rettiwt();

async function getTimeline() {

  try {
    // 3002817576 is the ID for Finalmouse
    const timeline = await rettiwt.user.timeline('3002817576');
    if (timeline.list.length > 0) {
      const mostRecentTweet = timeline.list[0];
      const tweetUrl = `https://fxtwitter.com/finalmouse/status/${mostRecentTweet.id}`;
      console.log(tweetUrl);
    } else {
      console.log('No tweets found for that user');
    }
    
  } catch (error) {
    if (error.response) {
      // The request was made and the server responded with a status code
      // that falls out of the range of 2xx
      console.log(error.response.data);
      console.log(error.response.status);
      console.log(error.response.headers);
    } else if (error.request) {
      // The request was made but no response was received
      console.log(error.request);
    } else {
      // Something happened in setting up the request that triggered an Error
      console.log('Error', error.message);
    }
    console.log(error.config);
  }
}

getTimeline();
