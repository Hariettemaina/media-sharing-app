document.addEventListener('DOMContentLoaded', function () {
    const uploadForm = document.getElementById('upload-form');
    const fileInput = document.getElementById('file-input');
    const mediaGrid = document.querySelector('.media-grid'); 

    uploadForm.addEventListener('submit', function (event) {
        event.preventDefault();

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

        fetch('http://localhost:8000', {
            method: 'POST',
            body: formData,
        })
            .then(response => response.json())
            .then(data => {
                console.log('Full response;', data);
                if (data.data && data.data.images.upload) {
                    const imageData = data.data.images.upload;

                    const imgElement = document.createElement('img');
                    imgElement.src = imageData;
                    imgElement.style.width = '200px';

                    mediaGrid.appendChild(imgElement);
                    const storedimages = JSON.parse(localStorage.getItem('uploadedimages')) || [];
                    storedimages.push(imageData);
                    localStorage.setItem('uploadedimages', JSON.stringify(storedimages));
                } else {
                    console.error('Upload failed', data);
                }
            })
            .catch(error => {
                console.error('Error:', error);
            });
    });

    function loadStoredImages() {
        const storedImages = JSON.parse(localStorage.getItem('uploadedimages')) || [];
        storedImages.forEach(url => {
            const imageElement = document.createElement('img');
            imageElement.src = url;
            imageElement.style.width = '200px';
            mediaGrid.appendChild(imageElement);
        });
    }
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
