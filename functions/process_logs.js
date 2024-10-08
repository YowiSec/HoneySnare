const { execFile } = require('child_process');
const path = require('path');

exports.handler = async function(event, context) {
  return new Promise((resolve, reject) => {
    const rustBinary = path.join(__dirname, 'backend');
    execFile(rustBinary, (error, stdout, stderr) => {
      if (error) {
        reject({ statusCode: 500, body: JSON.stringify({ error: error.message }) });
      } else {
        resolve({
          statusCode: 200,
          body: JSON.stringify({ logs: stdout.split('\n').filter(Boolean) })
        });
      }
    });
  });
};