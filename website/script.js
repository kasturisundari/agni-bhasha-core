// Copy Code Functionality
function copyCode() {
    const codeElement = document.getElementById('sutra-code');
    const textArea = document.createElement('textarea');
    textArea.value = codeElement.textContent;
    document.body.appendChild(textArea);
    textArea.select();
    
    try {
        document.execCommand('copy');
        const copyBtn = document.querySelector('.copy-btn');
        const originalText = copyBtn.innerHTML;
        copyBtn.innerHTML = '<i class="fas fa-check"></i> Copied!';
        copyBtn.style.color = '#4CAF50';
        
        setTimeout(() => {
            copyBtn.innerHTML = originalText;
            copyBtn.style.color = '';
        }, 2000);
    } catch (err) {
        console.error('Failed to copy text', err);
    }
    
    document.body.removeChild(textArea);
}

// Simple Scroll Reveal Animation
document.addEventListener("DOMContentLoaded", () => {
    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                entry.target.style.opacity = 1;
                entry.target.style.transform = 'translateY(0)';
            }
        });
    }, { threshold: 0.1 });

    const sections = document.querySelectorAll('.section');
    sections.forEach(section => {
        section.style.opacity = 0;
        section.style.transform = 'translateY(20px)';
        section.style.transition = 'all 0.8s ease-out';
        observer.observe(section);
    });
});

// Live Compiler Logic
function compileSutra() {
    const code = document.getElementById('sutra-editor').value;
    const output = document.getElementById('compiler-output');
    
    output.innerHTML = '<i>[प्रक्रिया]</i> अष्टाध्यायी-व्याकरण-नियमानां विश्लेषणं क्रियते...<br>';
    
    setTimeout(() => {
        output.innerHTML += '<i>[प्रक्रिया]</i> लिङ्ग-नियमानां प्रमाणीकरणं क्रियते...<br>';
        
        setTimeout(() => {
            if (code.includes('koṣā = 0') || code.includes('koṣā =') && code.includes('Strilinga')) {
                output.innerHTML += '<br><span class="error">[✗] FATAL ERROR: Reentrancy Attack Detected!</span><br>';
                output.innerHTML += '<span class="error">Variable ending in \'ā\' (koṣā) is Strilinga (Feminine) and is immutable by external scopes.</span><br>';
                output.innerHTML += '<span class="success">√vacति·"SUCCESS: Attack blocked by Linganushasanam!"</span>';
            } else {
                output.innerHTML += '<br><span class="success">[✓] Compilation Successful!</span><br>';
                output.innerHTML += '<span>Contract deployed to local VM state.</span>';
            }
        }, 800);
    }, 600);
}

// Wallet & Faucet Logic
document.addEventListener("DOMContentLoaded", () => {
    const connectBtn = document.getElementById('connect-wallet-btn');
    if(connectBtn) {
        connectBtn.addEventListener('click', () => {
            if (connectBtn.innerText.includes('मुद्राकोशं योजयतु') || connectBtn.innerText.includes('Connect Wallet')) {
                connectBtn.innerHTML = '<i class="fas fa-spinner fa-spin"></i> योज्यते...';
                
                setTimeout(() => {
                    const mockAddress = '0xKast...' + Math.floor(Math.random() * 1000) + 'f2';
                    connectBtn.innerHTML = `<i class="fas fa-wallet"></i> ${mockAddress}`;
                    connectBtn.classList.remove('btn-primary');
                    connectBtn.classList.add('btn-outline');
                    
                    showToast('मुद्राकोशः साफल्येन संयोजितः!', 'fa-check-circle');
                    
                    // Trigger Faucet 2 seconds later
                    setTimeout(() => {
                        showToast('धारा: १०० $KST मुद्राः प्राप्ताः!', 'fa-coins');
                    }, 2000);
                }, 1500);
            }
        });
    }
});

function showToast(message, iconClass) {
    const container = document.getElementById('toast-container');
    if (!container) return;
    
    const toast = document.createElement('div');
    toast.className = 'toast';
    toast.innerHTML = `<i class="fas ${iconClass}"></i> <span>${message}</span>`;
    
    container.appendChild(toast);
    
    setTimeout(() => {
        toast.classList.add('fade-out');
        setTimeout(() => {
            container.removeChild(toast);
        }, 500);
    }, 4000);
}

// ═══════════════════════════════════════════════════════════
// Cosmic Clock Logic
// ═══════════════════════════════════════════════════════════
const NAKSHATRAS = ["Ashvini", "Bharani", "Krittika", "Rohini", "Mrigashirsha", "Ardra", "Punarvasu", "Pushya", "Ashlesha", "Magha", "Purva Phalguni", "Uttara Phalguni", "Hasta", "Chitra", "Swati", "Vishakha", "Anuradha", "Jyeshtha", "Mula", "Purva Ashadha", "Uttara Ashadha", "Shravana", "Dhanishta", "Shatabhisha", "Purva Bhadrapada", "Uttara Bhadrapada", "Revati"];

function startCosmicClock() {
    const nakshatraEl = document.getElementById('current-nakshatra');
    const countdownEl = document.getElementById('block-countdown');
    
    if (!nakshatraEl || !countdownEl) return;

    let currentNakshatraIdx = Math.floor(Math.random() * NAKSHATRAS.length);
    let blockTimeRemaining = 3.0; // Simulated 3 seconds per block

    setInterval(() => {
        blockTimeRemaining -= 0.1;
        
        if (blockTimeRemaining <= 0) {
            // Block mined!
            blockTimeRemaining = 3.0;
            currentNakshatraIdx = (currentNakshatraIdx + 1) % NAKSHATRAS.length;
            nakshatraEl.textContent = NAKSHATRAS[currentNakshatraIdx];
            
            // Add a subtle flash effect
            nakshatraEl.style.color = '#fff';
            nakshatraEl.style.textShadow = '0 0 20px #fff';
            setTimeout(() => {
                nakshatraEl.style.color = '';
                nakshatraEl.style.textShadow = '';
            }, 300);
        }
        
        // Format to 2 decimal places
        countdownEl.textContent = `Next Block: ${blockTimeRemaining.toFixed(2)} Muhurta`;
    }, 100);
}

// Start clock on load
document.addEventListener("DOMContentLoaded", startCosmicClock);

// ═══════════════════════════════════════════════════════════
// Visual AST Builder (Live Parser)
// ═══════════════════════════════════════════════════════════
function initASTVisualizer() {
    const editor = document.getElementById('sutra-editor');
    if (!editor) return;

    editor.addEventListener('input', () => {
        parseAndRenderAST(editor.value);
    });
    
    // Initial render
    parseAndRenderAST(editor.value);
}

function parseAndRenderAST(code) {
    const canvas = document.getElementById('ast-canvas');
    if (!canvas) return;

    if (!code.trim()) {
        canvas.innerHTML = '<div class="ast-placeholder">Type in the editor to visualize the AST</div>';
        return;
    }

    let treeHTML = '<div class="ast-tree">';
    let nodesFound = false;

    // Detect Strilinga Assignment (Variables ending in ā or ī)
    const assignmentMatches = [...code.matchAll(/([a-zA-Z_āīśṣ]+[āī])\s*=\s*([^\\n]+)/g)];
    for (const match of assignmentMatches) {
        if (nodesFound) treeHTML += '<div class="ast-spacer"></div>';
        nodesFound = true;
        const varName = match[1];
        const val = match[2].trim();
        treeHTML += `
            <div class="ast-node root-node">
                <span class="ast-label">Assignment (Karma)</span>
                <span class="ast-val">=</span>
            </div>
            <div class="ast-children">
                <div class="ast-child-wrapper">
                    <div class="ast-line"></div>
                    <div class="ast-node strilinga-node glow-anim">
                        <span class="ast-label">Strilinga (Immutable)</span>
                        <span class="ast-val">${varName}</span>
                    </div>
                </div>
                <div class="ast-child-wrapper">
                    <div class="ast-line"></div>
                    <div class="ast-node">
                        <span class="ast-label">Value (Artha)</span>
                        <span class="ast-val">${val}</span>
                    </div>
                </div>
            </div>
        `;
    }

    // Detect Dhatu Call (e.g., √vacति·"SUCCESS...")
    const dhatuMatches = [...code.matchAll(/√([a-zA-Z]+)([a-zA-Zāīūṛṝḷḹēōaiōauṃḥ]+)·"([^"]+)"/g)];
    for (const match of dhatuMatches) {
        if (nodesFound) treeHTML += '<div class="ast-spacer"></div>';
        nodesFound = true;
        const root = match[1];
        const suffix = match[2];
        const arg = match[3];
        
        treeHTML += `
            <div class="ast-node root-node">
                <span class="ast-label">Dhatu Invocation (Kriya)</span>
                <span class="ast-val">√${root}</span>
            </div>
            <div class="ast-children">
                <div class="ast-child-wrapper">
                    <div class="ast-line"></div>
                    <div class="ast-node dhatu-node glow-anim-alt">
                        <span class="ast-label">Pratyaya (Suffix)</span>
                        <span class="ast-val">${suffix}</span>
                    </div>
                </div>
                <div class="ast-child-wrapper">
                    <div class="ast-line"></div>
                    <div class="ast-node">
                        <span class="ast-label">Argument (Karaka)</span>
                        <span class="ast-val">"${arg}"</span>
                    </div>
                </div>
            </div>
        `;
    }

    if (!nodesFound) {
        treeHTML = '<div class="ast-placeholder">Typing... Waiting for valid Sutra grammar (e.g. koṣā = 100 or √vacति)</div>';
    } else {
        treeHTML += '</div>'; // close ast-tree
    }

    canvas.innerHTML = treeHTML;
}

// Init AST Visualizer on load
document.addEventListener("DOMContentLoaded", initASTVisualizer);

// ═══════════════════════════════════════════════════════════
// Forum Logic (Publisher Modal)
// ═══════════════════════════════════════════════════════════
function openPublisherModal() {
    const modal = document.getElementById('publisher-modal');
    if (modal) {
        modal.classList.add('active');
        document.body.style.overflow = 'hidden'; // prevent scrolling behind
    }
}

function closePublisherModal() {
    const modal = document.getElementById('publisher-modal');
    if (modal) {
        modal.classList.remove('active');
        document.body.style.overflow = '';
    }
}

function submitPost() {
    showToast("Post submitted to the network successfully!", "fa-check-circle");
    setTimeout(closePublisherModal, 1000);
}

// Close modal if clicked outside the content box
document.addEventListener('click', (e) => {
    const modal = document.getElementById('publisher-modal');
    if (modal && e.target === modal) {
        closePublisherModal();
    }
});
