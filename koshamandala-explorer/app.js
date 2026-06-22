// KasturiScan Live Data Fetcher (Zero Mocks)
const RPC_URL = "https://kasturisundari.xyz/samparka";

let currentBlock = 0;
let currentTps = 0;

async function fetchRealNetworkData() {
    try {
        const response = await fetch(RPC_URL, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({
                "jsonrpc": "2.0",
                "method": "kosha_sthiti",
                "params": [],
                "id": Date.now()
            })
        });
        
        const data = await response.json();
        if (data && data.result) {
            updateDashboard(data.result);
        }
    } catch (e) {
        console.error("Failed to fetch real data from Samparka Node:", e);
    }
}

function updateDashboard(nodeData) {
    // nodeData contains actual block size, TPS, and mempool size
    currentBlock = nodeData.kosha_size || 0;
    currentTps = nodeData.tps || 0;
    
    document.getElementById('latestBlock').innerText = currentBlock.toLocaleString();
    document.getElementById('liveTps').innerText = currentTps.toLocaleString();
    
    // Populate real blocks if available
    const tbody = document.getElementById('blocksBody');
    tbody.innerHTML = ''; // Clear previous
    if (nodeData.recent_blocks && nodeData.recent_blocks.length > 0) {
        nodeData.recent_blocks.forEach(block => {
            const row = document.createElement('tr');
            row.innerHTML = `
                <td><span class="hash">${block.index}</span></td>
                <td>${new Date(block.timestamp * 1000).toLocaleTimeString()}</td>
                <td>${block.transactions}</td>
                <td><span class="hash">${block.miner.substring(0, 16)}...</span></td>
            `;
            tbody.appendChild(row);
        });
    } else {
        tbody.innerHTML = '<tr><td colspan="4">No blocks mined yet. Network awaiting Seva contributions.</td></tr>';
    }

    // Populate real mempool transactions
    const txBody = document.getElementById('txBody');
    txBody.innerHTML = '';
    if (nodeData.mempool && nodeData.mempool.length > 0) {
        nodeData.mempool.forEach(tx => {
            const row = document.createElement('tr');
            row.innerHTML = `
                <td><span class="hash">${tx.hash.substring(0, 14)}...</span></td>
                <td><span class="hash">${tx.from.substring(0, 10)}...</span></td>
                <td><span class="hash">${tx.to.substring(0, 10)}...</span></td>
                <td>${tx.amount}</td>
            `;
            txBody.appendChild(row);
        });
    } else {
        txBody.innerHTML = '<tr><td colspan="4">Mempool empty.</td></tr>';
    }
}

// Fetch real data every 5 seconds
setInterval(fetchRealNetworkData, 5000);
fetchRealNetworkData();

document.getElementById('searchBtn').addEventListener('click', () => {
    const query = document.getElementById('searchInput').value;
    if(query) alert(`Data DataSearch Data Data Data RPC Data: ${query}`);
});
