console.log('Loading function');

const aws = require('aws-sdk');
const ffmpegPath = require('@ffmpeg-installer/ffmpeg').path;
const ffmpeg = require('fluent-ffmpeg');
ffmpeg.setFfmpegPath(ffmpegPath);

const s3 = new aws.S3();
const fs = require('fs');

exports.handler = async (event, context) => {
    //console.log('Received event:', JSON.stringify(event, null, 2));

    // Get the object from the event and show its content type
    const srcBucket = event.Records[0].s3.bucket.name;
    const srcKey = decodeURIComponent(event.Records[0].s3.object.key.replace(/\+/g, ' '));

    const dstBucket = srcBucket + '-processed';
    const dstsKey = 'processed-' + srcKey;
    const srcExtension = srcKey.slice(srcKey.lastIndexOf('.')).split('.')[1];
    const srcFilename = srcKey.slice(0, srcKey.lastIndexOf('.'))

    

    try { 
        // if (srcExtension === 'temp') {
        const params = {
            Bucket: srcBucket,
            Key: srcKey
        };
        const srcFile = await s3.getObject(params).promise();
        const outputFile = srcFilename + '.mp3';

        ffmpeg(srcFile).audioCodec('libmp3lame')
            .audioBitrate(192)
            .audioChannels(2)
            .on('end', function() {
                fs.readFile(outputFile, async (e, data) => {
                    const destParams = {
                        Bucket: dstBucket,
                        Key: outputFile,
                        Body: data,
                        ContentType: 'audio/mp3'
                    };
                    console.log('uhhhh')
                    const putResult = await s3.putObject(destParams).promise();
                })
            })
            .output(outputFile)
            .run()

        // }
        
        

    } catch (error) {
        console.log(error)
        return;
    }

    

    // try {
    //     const { ContentType } = await s3.getObject(params).promise();
    //     console.log('CONTENT TYPE:', ContentType);
    //     return ContentType;
    // } catch (err) {
    //     console.log(err);
    //     const message = `Error getting object ${key} from bucket ${bucket}. Make sure they exist and your bucket is in the same region as this function.`;
    //     console.log(message);
    //     throw new Error(message);
    // }
};
