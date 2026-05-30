(function () {
  'use strict';

  // svg icons for copy and check states
  const copyIcon = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect><path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path></svg>`;
  const checkIcon = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"></polyline></svg>`;

  /**
   * Creates and adds copy buttons to all code blocks
   */
  function addCopyButtons() {
    // select all code blocks - Hugo wraps highlighted code in .highlight > pre > code
    const codeBlocks = document.querySelectorAll('.highlight');

    codeBlocks.forEach(function (block) {
      // skip if already has a copy button
      if (block.querySelector('.copy-button')) {
        return;
      }

      // create the copy button
      const button = document.createElement('button');
      button.className = 'copy-button';
      button.type = 'button';
      button.setAttribute('aria-label', 'Copy code to clipboard');
      button.innerHTML = copyIcon + '<span>Copy</span>';

      // add click handler
      button.addEventListener('click', function () {
        copyCode(block, button);
      });

      // insert button into the highlight container
      block.insertBefore(button, block.firstChild);
    });

    // also handle standalone pre blocks (not wrapped in .highlight)
    const standalonePres = document.querySelectorAll('pre:not(.highlight pre)');
    standalonePres.forEach(function (pre) {
      // skip if parent is .highlight (already handled)
      if (pre.parentElement && pre.parentElement.classList.contains('highlight')) {
        return;
      }
      
      // skip if already has a copy button
      if (pre.querySelector('.copy-button')) {
        return;
      }

      // wrap pre in a container for positioning
      const wrapper = document.createElement('div');
      wrapper.className = 'highlight';
      pre.parentNode.insertBefore(wrapper, pre);
      wrapper.appendChild(pre);

      // create the copy button
      const button = document.createElement('button');
      button.className = 'copy-button';
      button.type = 'button';
      button.setAttribute('aria-label', 'Copy code to clipboard');
      button.innerHTML = copyIcon + '<span>Copy</span>';

      // add click handler
      button.addEventListener('click', function () {
        copyCode(wrapper, button);
      });

      // insert button
      wrapper.insertBefore(button, wrapper.firstChild);
    });
  }

  /**
   * Copies the code content to clipboard
   * @param {HTMLElement} block - The code block container
   * @param {HTMLElement} button - The copy button element
   */
  function copyCode(block, button) {
    // get the code element
    const code = block.querySelector('code');
    if (!code) {
      return;
    }

    // get the text content (without any line numbers if present)
    const text = code.innerText || code.textContent;

    // use the Clipboard API
    navigator.clipboard.writeText(text).then(function () {
      // success - update button state
      button.classList.add('copied');
      button.innerHTML = checkIcon + '<span>Copied!</span>';

      // reset after 2 seconds
      setTimeout(function () {
        button.classList.remove('copied');
        button.innerHTML = copyIcon + '<span>Copy</span>';
      }, 2000);
    }).catch(function (err) {
      // fallback for older browsers
      console.error('Failed to copy:', err);
      fallbackCopy(text, button);
    });
  }

  /**
   * Fallback copy method for browsers without Clipboard API
   * @param {string} text - Text to copy
   * @param {HTMLElement} button - The copy button element
   */
  function fallbackCopy(text, button) {
    const textarea = document.createElement('textarea');
    textarea.value = text;
    textarea.style.position = 'fixed';
    textarea.style.left = '-9999px';
    textarea.style.top = '0';
    document.body.appendChild(textarea);
    textarea.select();

    try {
      document.execCommand('copy');
      button.classList.add('copied');
      button.innerHTML = checkIcon + '<span>Copied!</span>';

      setTimeout(function () {
        button.classList.remove('copied');
        button.innerHTML = copyIcon + '<span>Copy</span>';
      }, 2000);
    } catch (err) {
      console.error('Fallback copy failed:', err);
    }

    document.body.removeChild(textarea);
  }

  // initialize when DOM is ready
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', addCopyButtons);
  } else {
    addCopyButtons();
  }
})();
