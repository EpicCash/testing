ps aux | grep epic | grep listen | awk '{print $2}' | xargs kill -9
