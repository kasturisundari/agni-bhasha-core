/**
 * Kasturi3.js
 * The Official Web3 Provider SDK for KasturiChain Sovereign Network
 */

class KasturiProvider {
    constructor(rpcUrl = "https://kasturisundari.xyz/samparka") {
        this.rpcUrl = rpcUrl;
    }

    async send(method, params = []) {
        try {
            const response = await fetch(this.rpcUrl, {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({
                    jsonrpc: "2.0",
                    method: method,
                    params: params,
                    id: Date.now()
                })
            });
            const data = await response.json();
            if (data.error) throw new Error(data.error.message);
            return data.result;
        } catch (error) {
            console.error(`[Kasturi3 Error] ${method}:`, error);
            throw error;
        }
    }

    /**
     * Get the current Nakshatra (Cosmic Time)
     */
    async getCurrentNakshatra() {
        return await this.send("nakshatra_kala");
    }

    /**
     * Get the entire blockchain state (Blocks)
     */
    async getBlocks() {
        return await this.send("shringkhala_sthiti");
    }

    /**
     * Get Database / Kosha status
     */
    async getNetworkStatus() {
        return await this.send("kosha_sthiti");
    }

    /**
     * Evaluate arbitrary Agni Bhasha (Sutra) Code on the Node Sandbox
     * @param {string} code The Sutra code to evaluate
     */
    async executeSutra(code) {
        return await this.send("sutra_eval", [code]);
    }

    /**
     * Send a raw transaction to the network
     * This evaluates the transaction logic on the node, effectively putting it in the Mempool/State
     */
    async sendTransaction(sutraScript) {
        return await this.executeSutra(sutraScript);
    }
}

// Attach to window globally to mimic window.ethereum
if (typeof window !== 'undefined') {
    window.kasturi3 = new KasturiProvider();
}

export default KasturiProvider;
