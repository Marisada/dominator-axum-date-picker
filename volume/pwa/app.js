const VERSION = '660731-1515'

import init from './client.js'
init('./client_bg.wasm').catch(console.error)

if ('serviceWorker' in navigator) {
    const checkUpdate = document.getElementById('checkUpdate')
    const updateBanner = document.getElementById('updateBanner')
    const bannerContent = document.querySelector('#updateBanner .banner-content')

    checkUpdate.style.display = 'block'

    navigator.serviceWorker.register('/sw.js').then(reg => {
        reg.onupdatefound = () => {
            const installingWorker = reg.installing
            if (installingWorker === null) return
            installingWorker.onstatechange = () => {
                if (installingWorker.state === 'installed') {
                    if (navigator.serviceWorker.controller !== null) {
                        updateBanner.dataset.state = 'updateavailable'
                        document.querySelector('#updateBanner .banner-headline').innerHTML = 'Update Available'
                        document.querySelector('#updateBanner .banner-subhead').innerHTML = 'App is updating to the latest version..'
                        updateBanner.style.height = bannerContent.offsetHeight + 'px'
                        setTimeout(() => window.location.reload(), 3000)
                    }
                }
            }
        }
        checkUpdate.onclick = () => {
            checkUpdate.style.display = 'none'
            updateBanner.style.height = bannerContent.offsetHeight + 'px'
            reg.update()
            setTimeout(() => {
                updateBanner.style.height = '0'
                checkUpdate.style.display = 'block'
            }, 3000)
        }
    }).catch(err => console.error('Service Worker Registration : ' + err))

    navigator.serviceWorker.ready.then(async reg => {
        if (reg.active !== null) {
            reg.active.postMessage({type:'version', value: VERSION})
        }
    }).catch(err => console.error('Service Worker Ready : ' + err))
}