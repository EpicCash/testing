ps aux | grep epic | grep usernet | awk '{print $2}' | xargs kill -9
