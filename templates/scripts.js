document.addEventListener('DOMContentLoaded', function () {
    const uploadForm = document.getElementById('upload-form');
    const fileInput = document.getElementById('file-input');
    const mediaGrid = document.querySelector('.media-grid');

    let websocket;

    function initWebSocket() {
        websocket = new WebSocket('ws://localhost:8080');

        websocket.onopen = function () {
            console.log('WebSocket connection established');
        };

        websocket.onmessage = function (event) {
            const message = JSON.parse(event.data);
            handleWebSocketMessage(message);
        };


        websocket.onclose = function () {
            console.log('WebSocket connection closed');
        };


        websocket.onerror = function (error) {
            console.error('WebSocket error:', error);
        };
    }

    // Handle a WebSocket message
    function handleWebSocketMessage(message) {
        // Check the type of the message and take appropriate action
        if (message.type === 'uploadStatus') {
            console.log('Upload status:', message.data);
        } else if (message.type === 'newImage') {
            appendImageToGrid(message.data.imageUrl);
            storeImageInSession(message.data.imageUrl);
        }
    }


    uploadForm.addEventListener('submit', async function (event) {
        event.preventDefault();

        // Check if a file is selected
        if (!fileInput.files.length) {
            alert('Please select a file to upload.');
            return;
        }


        const formData = new FormData();
        formData.append('operations', JSON.stringify({
            query: `
                mutation UploadFile($input: UploadUserInput!) {
                    images {
                        upload(input: $input)
                    }
                }
            `,
            variables: {
                input: {
                    image: null,
                    userId: 1
                }
            }
        }));
        formData.append('map', JSON.stringify({
            '0': ['variables.input.image']
        }));
        formData.append('0', fileInput.files[0]);

        try {
            // Send a POST request to the server with the form data
            const response = await fetch('http://localhost:8080', {
                method: 'POST',
                body: formData,
            });
            const data = await response.json();
            console.log('Full response:', data);

            // Check if the upload was successful
            if (data.errors) {
                console.error('Upload failed:', data.errors);
                alert('Upload failed. Please try again.');
            } else if (data.data && data.data.images.upload) {
                const imageData = data.data.images.upload;

                // Append the image to the grid
                appendImageToGrid(imageData);
                // Store the image in the session
                storeImageInSession(imageData);
                // Send a WebSocket message with the image data
                if (websocket.readyState === WebSocket.OPEN) {
                    websocket.send(JSON.stringify({ type: 'newUpload', data: imageData }));
                } else {
                    console.error('WebSocket connection not open');
                }
            }
        } catch (error) {
            console.error('Error:', error);
            alert('Error uploading the file. Please try again.');
        }
    });


    function loadStoredImages() {
        const storedImages = JSON.parse(sessionStorage.getItem('uploadedimages')) || [];
        console.log('Retrieved stored images:', storedImages);

        storedImages.forEach(appendImageToGrid);
    }


    function appendImageToGrid(imageUrl) {
        const imgElement = document.createElement('img');
        imgElement.src = imageUrl;
        imgElement.style.width = '200px';
        mediaGrid.appendChild(imgElement);
    }

    function storeImageInSession(imageUrl) {
        const storedImages = JSON.parse(sessionStorage.getItem('uploadedimages')) || [];
        storedImages.push(imageUrl);
        sessionStorage.setItem('uploadedimages', JSON.stringify(storedImages));
        console.log('Stored images:', storedImages);
    }


    initWebSocket();
    loadStoredImages();
});






// function getUserIdFromCookie() {
//     var key, value, i;
//     var cookieArray = document.cookie.split(';');

//     for (i = 0; i < cookieArray.length; i++) {
//         key = cookieArray[i].slice(0, cookieArray[i].indexOf("="));
//         value = cookieArray[i].slice(cookieArray[i].indexOf("=") + 1);

//         if (key == 'id') {
//             return parseInt(value, 10);
//         }
//     }
//     return null;
// }

// const id = getUserIdFromCookie();
// if (id === null) {
//     console.error('User ID not found in cookies.');
//     return;
// }




// 4. *Backpressure Handling with RabbitMQ*
//    - *Description:* Integrate RabbitMQ to handle backpressure when processing media files.
//    - *Expected Functionality:* RabbitMQ queues are used to manage the flow of media processing tasks. Workers consume tasks from the queue and process them asynchronously.
//    - *Criteria for Completion:* RabbitMQ is integrated into the system to handle backpressure effectively.
//    - *Test Suites:*
//      - Test RabbitMQ integration.
//      - Test asynchronous processing of media tasks.

// 5. *Real-time Updates with WebSockets*
//    - *Description:* Implement real-time updates for users using WebSockets.
//    - *Expected Functionality:* Users receive real-time notifications when new media is uploaded or when there are updates to their uploaded media.
//    - *Criteria for Completion:* Real-time updates are implemented using WebSockets, and users receive notifications in real-time.
//    - *Test Suites:*
//      - Test WebSocket connection establishment.
//      - Test real-time notifications for media uploads and updates.







