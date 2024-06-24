document.addEventListener('DOMContentLoaded', function () {
    const uploadForm = document.getElementById('upload-form');
    const fileInput = document.getElementById('file-input');
    const mediaGrid = document.querySelector('.media-grid');

    let websocket;

    function initWebSocket() {
        websocket = new WebSocket('ws://localhost:8080', 'graphql-ws');

        websocket.onopen = function () {
            console.log('WebSocket connection established');
            const payload = {
                type: 'connection_init',
                payload: {}
            };
            websocket.send(JSON.stringify(payload));

            // Start subscription only after the connection is open
            startSubscription();
        };

        websocket.onmessage = function (event) {
            console.log('WebSocket message received:', event.data); // Log received message for debugging
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

    function handleWebSocketMessage(message) {
        console.log('Handling WebSocket message:', message); // Log the message for debugging
        if (message.type === 'data' && message.id === '1') {
            const mediaUpdate = message.payload.data.mediaUpdates;
            const imageUrl = extractImageUrl(mediaUpdate.message);
            appendImageToGrid(imageUrl);
        } else if (message.type === 'notification' && message.id === 'new_upload') {
            alert(`New image uploaded by user ${message.payload.data.userId}`);
        }
    }

    function extractImageUrl(message) {
        // Extract the image URL from the message
        const matches = message.match(/Path: (.+?) /);
        if (matches && matches[1]) {
            return `http://localhost:8080/${matches[1].trim()}`;
        }
        return '';
    }

    function startSubscription() {
        const subscriptionQuery = {
            id: '1',
            type: 'start',
            payload: {
                query: `
                    subscription {
                        mediaUpdates {
                            userId
                            message
                        }
                    }
                `,
                variables: {}
            }
        };
        websocket.send(JSON.stringify(subscriptionQuery));
    }

    uploadForm.addEventListener('submit', async function (event) {
        event.preventDefault();

        const userId = sessionStorage.getItem("userId");
        if (!userId) {
            alert('Please login first');
            return;
        }

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
                    userId: parseInt(userId)
                }
            }
        }));
        formData.append('map', JSON.stringify({ '0': ['variables.input.image'] }));
        formData.append('0', fileInput.files[0]);

        try {
            const response = await fetch('http://localhost:8080', {
                method: 'POST',
                body: formData
            });
            const data = await response.json();

            if (data.errors) {
                console.error('Upload failed:', data.errors);
                alert('Upload failed. Please try again.');
            } else if (data.data && data.data.images.upload) {
                const imageData = data.data.images.upload;
                appendImageToGrid(imageData);
                storeImageInSession(imageData);
            }
        } catch (error) {
            console.error('Error:', error);
            alert('Error uploading the file. Please try again.');
        }
    });

    function loadStoredImages() {
        const storedImages = JSON.parse(sessionStorage.getItem('uploadedimages')) || [];
        storedImages.forEach(appendImageToGrid);
    }

    function appendImageToGrid(imageUrl) {
        if (imageUrl) {
            const imgElement = document.createElement('img');
            imgElement.src = imageUrl;
            imgElement.style.width = '200px';
            mediaGrid.appendChild(imgElement);
        }
    }

    function storeImageInSession(imageUrl) {
        const storedImages = JSON.parse(sessionStorage.getItem('uploadedimages')) || [];
        storedImages.push(imageUrl);
        sessionStorage.setItem('uploadedimages', JSON.stringify(storedImages));
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







