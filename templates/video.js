document.addEventListener('DOMContentLoaded', function () {
    const uploadForm = document.getElementById('upload-form');
    const fileInput = document.getElementById('file-input');

    uploadForm.addEventListener('submit', function (event) {
        event.preventDefault();

        const formData = new FormData();
        formData.append('operations', JSON.stringify({
            query: `
                mutation UploadVideo($input: VideoUserInput!) {
                    videos {
                        upload(input: $input)
                    }
                }
            `,
            variables: {
                input: {
                    video: null, 
                    userId: 1 
                }
            }
        }));
        formData.append('map', JSON.stringify({
            '0': ['variables.input.video']
        }));
        formData.append('0', fileInput.files[0]);

        fetch('http://localhost:8000', {
            method: 'POST',
            body: formData,
        })
            .then(response => response.json())
            .then(data => {
                if (data.data && data.data.videos && data.data.videos.upload) {
                    console.log('Upload successful');
                } else {
                    console.error('Upload failed', data);
                }
            })
            .catch(error => {
                console.error('Error:', error);
            });
    });
});
