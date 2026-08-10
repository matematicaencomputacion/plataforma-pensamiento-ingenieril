/**
 * IngenierIA — Web Speech API glue for coaching dictation (browser-only).
 */
(function (global) {
  "use strict";

  var recognition = null;
  var listening = false;

  function ctor() {
    return global.SpeechRecognition || global.webkitSpeechRecognition || null;
  }

  function isSupported() {
    return typeof ctor() === "function";
  }

  function compose(base, finals, interim) {
    var spoken = String(finals || "") + String(interim || "");
    if (!spoken) {
      return String(base || "");
    }
    if (!base) {
      return spoken.replace(/^\s+/, "");
    }
    if (/\s$/.test(base) || /^\s/.test(spoken)) {
      return base + spoken;
    }
    return base + " " + spoken.replace(/^\s+/, "");
  }

  function voiceErrorMessage(code) {
    switch (code) {
      case "not-allowed":
      case "service-not-allowed":
        return "No pudimos acceder al micrófono. Revisá los permisos del navegador.";
      case "no-speech":
        return "No detectamos habla. Tocá el micrófono e intentá de nuevo.";
      case "audio-capture":
        return "No encontramos un micrófono disponible.";
      case "network":
        return "Falló la conexión del reconocimiento de voz.";
      case "aborted":
        return "";
      default:
        return "Hubo un problema con el reconocimiento de voz. Probá de nuevo.";
    }
  }

  function stop() {
    listening = false;
    if (!recognition) {
      return;
    }
    try {
      recognition.onresult = null;
      recognition.onerror = null;
      recognition.onend = null;
      recognition.stop();
    } catch (_err) {
      /* ignore */
    }
    recognition = null;
  }

  /**
   * @param {object} opts
   * @param {string} opts.base
   * @param {string} [opts.lang]
   * @param {(text: string) => void} opts.onUpdate
   * @param {(msg: string) => void} [opts.onError]
   * @param {() => void} [opts.onEnd]
   */
  function start(opts) {
    opts = opts || {};
    var Ctor = ctor();
    if (!Ctor) {
      if (opts.onError) {
        opts.onError("Tu navegador no soporta dictado por voz.");
      }
      return { ok: false };
    }

    stop();

    var base = String(opts.base || "");
    var finals = "";
    var instance = new Ctor();
    recognition = instance;
    listening = true;
    instance.continuous = true;
    instance.interimResults = true;
    instance.lang = opts.lang || "es-AR";

    instance.onresult = function (event) {
      var interim = "";
      for (var i = event.resultIndex; i < event.results.length; i += 1) {
        var result = event.results[i];
        var chunk = result[0] && result[0].transcript ? result[0].transcript : "";
        if (result.isFinal) {
          finals += chunk;
        } else {
          interim += chunk;
        }
      }
      if (opts.onUpdate) {
        opts.onUpdate(compose(base, finals, interim));
      }
    };

    instance.onerror = function (event) {
      var code = event && event.error ? event.error : "unknown";
      var msg = voiceErrorMessage(code);
      if (msg && opts.onError) {
        opts.onError(msg);
      }
      if (code !== "no-speech") {
        listening = false;
      }
    };

    instance.onend = function () {
      listening = false;
      recognition = null;
      if (opts.onEnd) {
        opts.onEnd();
      }
    };

    try {
      instance.start();
      return { ok: true };
    } catch (err) {
      listening = false;
      recognition = null;
      if (opts.onError) {
        opts.onError(err && err.message ? err.message : String(err));
      }
      return { ok: false };
    }
  }

  function isListening() {
    return listening;
  }

  global.ppiSpeech = {
    isSupported: isSupported,
    start: start,
    stop: stop,
    isListening: isListening,
    compose: compose,
    voiceErrorMessage: voiceErrorMessage,
  };
})(typeof window !== "undefined" ? window : globalThis);
