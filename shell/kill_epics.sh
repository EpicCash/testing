ps aux | grep epic | awk '{print $2}' | xargs kill -9
