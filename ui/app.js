const { invoke } = window.__TAURI__.core;

let state = {
    outputFolder: null,
    inputFile: null,
    isConverting: false,
    lastOutputPath: null
};

let logRefreshInterval = null;

const elements = {
    folderCard: document.getElementById('folderCard'),
    fileCard: document.getElementById('fileCard'),
    convertCard: document.getElementById('convertCard'),
    selectFolderBtn: document.getElementById('selectFolderBtn'),
    selectFileBtn: document.getElementById('selectFileBtn'),
    convertBtn: document.getElementById('convertBtn'),
    clearLogsBtn: document.getElementById('clearLogsBtn'),
    outputPath: document.getElementById('outputPath'),
    inputPath: document.getElementById('inputPath'),
    statusText: document.getElementById('statusText'),
    progressBar: document.getElementById('progressBar'),
    progressFill: document.getElementById('progressFill'),
    logContainer: document.getElementById('logContainer'),
    errorModal: document.getElementById('errorModal'),
    successModal: document.getElementById('successModal'),
    errorMessage: document.getElementById('errorMessage'),
    successMessage: document.getElementById('successMessage'),
    closeErrorBtn: document.getElementById('closeErrorBtn'),
    closeSuccessBtn: document.getElementById('closeSuccessBtn'),
    openFolderBtn: document.getElementById('openFolderBtn'),
};

async function init() {
    setupEventListeners();
    updateUI();
    startLogRefresh(2000);
}

function startLogRefresh(interval) {
    if (logRefreshInterval) {
        clearInterval(logRefreshInterval);
    }
    logRefreshInterval = setInterval(refreshLogs, interval);
}

function setupEventListeners() {
    elements.selectFolderBtn.addEventListener('click', selectOutputFolder);
    elements.selectFileBtn.addEventListener('click', selectInputFile);
    elements.convertBtn.addEventListener('click', runConversion);
    elements.clearLogsBtn.addEventListener('click', clearLogs);
    elements.closeErrorBtn.addEventListener('click', () => hideModal('errorModal'));
    elements.closeSuccessBtn.addEventListener('click', () => hideModal('successModal'));
    elements.openFolderBtn.addEventListener('click', openOutputFolder);
    
    document.querySelectorAll('.modal').forEach(modal => {
        modal.addEventListener('click', (e) => {
            if (e.target === modal) {
                hideModal(modal.id);
            }
        });
    });
}

async function selectOutputFolder() {
    try {
        const folder = await invoke('select_output_folder');
        if (folder) {
            state.outputFolder = folder;
            updateUI();
        }
    } catch (e) {
        showError(e.toString());
    }
}

async function selectInputFile() {
    try {
        const file = await invoke('select_input_file');
        if (file) {
            state.inputFile = file;
            updateUI();
        }
    } catch (e) {
        showError(e.toString());
    }
}

async function runConversion() {
    if (!state.outputFolder || !state.inputFile || state.isConverting) return;
    
    state.isConverting = true;
    updateUI();
    setStatus('processing', 'PROCESSING...');
    showProgress(true, true);
    startLogRefresh(300);
    await refreshLogs();
    
    try {
        const result = await invoke('run_conversion', {
            inputPath: state.inputFile,
            outputFolder: state.outputFolder
        });
        
        state.lastOutputPath = result.output_path;
        setStatus('success', 'CONVERSION COMPLETE');
        await refreshLogs();
        showSuccess(result.message);
    } catch (e) {
        setStatus('error', 'CONVERSION FAILED');
        await refreshLogs();
        showError(e.toString());
    } finally {
        state.isConverting = false;
        showProgress(false);
        updateUI();
        startLogRefresh(2000);
    }
}

function updateUI() {
    if (state.outputFolder) {
        elements.outputPath.textContent = shortenPath(state.outputFolder);
        elements.outputPath.classList.add('selected');
    } else {
        elements.outputPath.textContent = 'NOT SELECTED';
        elements.outputPath.classList.remove('selected');
    }
    
    if (state.inputFile) {
        elements.inputPath.textContent = shortenPath(state.inputFile);
        elements.inputPath.classList.add('selected');
    } else {
        elements.inputPath.textContent = 'NOT SELECTED';
        elements.inputPath.classList.remove('selected');
    }
    
    elements.selectFileBtn.disabled = !state.outputFolder;
    elements.convertBtn.disabled = !state.outputFolder || !state.inputFile || state.isConverting;
    
    if (!state.outputFolder) {
        setStatus('', 'SELECT OUTPUT FOLDER...');
    } else if (!state.inputFile) {
        setStatus('', 'SELECT INPUT FILE...');
    } else if (!state.isConverting) {
        setStatus('', 'READY TO CONVERT');
    }
    
    elements.convertBtn.textContent = state.isConverting ? 'PROCESSING...' : 'EXECUTE CONVERSION';
}

function shortenPath(path) {
    if (path.length > 50) {
        const parts = path.split(/[\\/]/);
        if (parts.length > 3) {
            return parts[0] + '/.../' + parts.slice(-2).join('/');
        }
    }
    return path;
}

function setStatus(type, message) {
    elements.statusText.className = 'card-value status-text';
    if (type) {
        elements.statusText.classList.add(type);
    }
    elements.statusText.textContent = message;
}

function showProgress(show, indeterminate = false) {
    if (show) {
        elements.progressBar.classList.add('visible');
        if (indeterminate) {
            elements.progressFill.classList.add('indeterminate');
        } else {
            elements.progressFill.classList.remove('indeterminate');
        }
    } else {
        elements.progressBar.classList.remove('visible');
        elements.progressFill.classList.remove('indeterminate');
    }
}

function showModal(modalId) {
    document.getElementById(modalId).classList.add('visible');
}

function hideModal(modalId) {
    document.getElementById(modalId).classList.remove('visible');
}

function showError(message) {
    elements.errorMessage.textContent = message;
    showModal('errorModal');
}

function showSuccess(message) {
    elements.successMessage.textContent = message;
    showModal('successModal');
}

async function openOutputFolder() {
    if (state.lastOutputPath) {
        try {
            await invoke('open_folder', { path: state.lastOutputPath });
        } catch (e) {
            console.error('Failed to open folder:', e);
        }
    }
    hideModal('successModal');
}

async function refreshLogs() {
    try {
        const logs = await invoke('get_logs');
        renderLogs(logs);
    } catch (e) {}
}

async function clearLogs() {
    try {
        await invoke('clear_logs');
        renderLogs([]);
    } catch (e) {
        console.error('Failed to clear logs:', e);
    }
}

function renderLogs(logs) {
    if (!logs || logs.length === 0) {
        elements.logContainer.innerHTML = '<div class="log-entry">[AWAITING SIGNAL...]</div>';
        return;
    }
    
    elements.logContainer.innerHTML = logs.map(log => {
        const date = new Date(log.timestamp * 1000);
        const time = date.toLocaleTimeString();
        return `<div class="log-entry"><span class="timestamp">[${time}]</span> <span class="level ${log.level}">${log.level}</span> ${escapeHtml(log.message)}</div>`;
    }).join('');
}

function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

document.addEventListener('DOMContentLoaded', init);
