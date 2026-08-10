/**
 * IngenierIA — motor Python client-side (Pyodide).
 * Port of frontend/src/lib/pyodide/engine.ts for the Leptos shell.
 * Solo browser. No ejecuta código en el servidor (ADR 002).
 */
(function (global) {
  "use strict";

  var PYODIDE_VERSION = "0.27.7";
  var PYODIDE_INDEX_URL =
    "https://cdn.jsdelivr.net/pyodide/v" + PYODIDE_VERSION + "/full/";

  var loadPromise = null;
  var instance = null;
  var lastError = null;

  function isBrowser() {
    return typeof window !== "undefined" && typeof document !== "undefined";
  }

  function injectPyodideScript() {
    if (global.loadPyodide) {
      return Promise.resolve(global.loadPyodide);
    }
    return new Promise(function (resolve, reject) {
      var script = document.createElement("script");
      script.src = PYODIDE_INDEX_URL + "pyodide.js";
      script.async = true;
      script.onload = function () {
        if (!global.loadPyodide) {
          reject(new Error("Pyodide cargó pero loadPyodide no está disponible"));
          return;
        }
        resolve(global.loadPyodide);
      };
      script.onerror = function () {
        reject(
          new Error("No se pudo cargar Pyodide desde " + PYODIDE_INDEX_URL),
        );
      };
      document.head.appendChild(script);
    });
  }

  function ensurePyodide() {
    if (!isBrowser()) {
      return Promise.reject(new Error("Pyodide solo está disponible en el navegador"));
    }
    if (instance) {
      return Promise.resolve(instance);
    }
    if (loadPromise) {
      return loadPromise;
    }
    loadPromise = injectPyodideScript()
      .then(function (loadPyodide) {
        return loadPyodide({ indexURL: PYODIDE_INDEX_URL });
      })
      .then(function (pyodide) {
        instance = pyodide;
        lastError = null;
        return pyodide;
      })
      .catch(function (err) {
        loadPromise = null;
        lastError = err && err.message ? err.message : String(err);
        throw err;
      });
    return loadPromise;
  }

  function collectIo(py) {
    var stdout = [];
    var stderr = [];
    py.setStdout({
      batched: function (text) {
        stdout.push(text);
      },
    });
    py.setStderr({
      batched: function (text) {
        stderr.push(text);
      },
    });
    return {
      join: function () {
        return {
          stdout: stdout.join("").replace(/\r\n/g, "\n"),
          stderr: stderr.join("").replace(/\r\n/g, "\n"),
        };
      },
    };
  }

  function buildCheckHarnessPython() {
    return [
      "import io",
      "import sys",
      "import types",
      "import traceback",
      "import importlib.util",
      "",
      "failures = []",
      "passed = 0",
      "",
      'spec = importlib.util.spec_from_file_location("student_tests", "test_step.py")',
      "mod = importlib.util.module_from_spec(spec)",
      "assert spec.loader is not None",
      "spec.loader.exec_module(mod)",
      "",
      "test_names = sorted(n for n in dir(mod) if n.startswith(\"test_\") and callable(getattr(mod, n)))",
      "if not test_names:",
      '    failures.append("No se encontró ninguna función test_* en el micro-reto.")',
      "",
      "for name in test_names:",
      "    fn = getattr(mod, name)",
      "    buf_out = io.StringIO()",
      "    buf_err = io.StringIO()",
      "    old_out, old_err = sys.stdout, sys.stderr",
      "    sys.stdout, sys.stderr = buf_out, buf_err",
      "",
      "    class Capsys:",
      "        def readouterr(self):",
      "            out = buf_out.getvalue()",
      "            err = buf_err.getvalue()",
      "            buf_out.seek(0)",
      "            buf_out.truncate(0)",
      "            buf_err.seek(0)",
      "            buf_err.truncate(0)",
      "            return types.SimpleNamespace(out=out, err=err)",
      "",
      "    try:",
      "        import inspect",
      "        params = inspect.signature(fn).parameters",
      "        kwargs = {}",
      '        if "capsys" in params:',
      "            kwargs[\"capsys\"] = Capsys()",
      "        fn(**kwargs)",
      "        passed += 1",
      '        print(f"PASSED {name}")',
      "    except Exception:",
      '        failures.append(f"FAILED {name}\\n{traceback.format_exc()}")',
      '        print(f"FAILED {name}")',
      "    finally:",
      "        sys.stdout, sys.stderr = old_out, old_err",
      "",
      "if failures:",
      '    print("---")',
      "    for f in failures:",
      "        print(f)",
      "else:",
      '    print(f"OK — {passed} test(s) passed")',
      "",
      "_CHECK_PASSED = len(failures) == 0",
    ].join("\n");
  }

  function statusMessage(status) {
    switch (status) {
      case "idle":
        return "Motor Python en espera.";
      case "loading":
        return "Preparando motor Python… (primera carga puede tardar unos segundos)";
      case "ready":
        return "Motor Python listo. Escribí en el editor y usá Run o Validar.";
      case "error":
        return (
          lastError ||
          "No se pudo iniciar Pyodide. Revisá la red o recargá la página."
        );
      default:
        return "Estado del motor desconocido.";
    }
  }

  function formatRunLog(result) {
    var parts = ["=== Run ==="];
    if (result.stdout && result.stdout.trim()) {
      parts.push(result.stdout.replace(/\n$/, ""));
    } else if (result.ok) {
      parts.push("(sin salida — usá print(...) para ver texto aquí)");
    }
    if (result.stderr && result.stderr.trim()) {
      parts.push("--- stderr ---");
      parts.push(result.stderr.replace(/\n$/, ""));
    }
    if (!result.ok && result.error) {
      parts.push("--- error ---");
      parts.push(result.error);
    }
    if (result.ok) {
      parts.push("");
      parts.push("✓ Ejecución finalizada");
    }
    return parts.join("\n");
  }

  function formatCheckLog(result) {
    var parts = ["=== Validar ===", result.summary || "", ""];
    if (result.details && result.details.trim()) {
      parts.push(result.details.replace(/\n$/, ""));
    }
    return parts.join("\n");
  }

  function ensure() {
    if (instance) {
      return Promise.resolve({
        status: "ready",
        message: statusMessage("ready"),
      });
    }
    return ensurePyodide()
      .then(function () {
        return { status: "ready", message: statusMessage("ready") };
      })
      .catch(function () {
        return { status: "error", message: statusMessage("error") };
      });
  }

  function run(code) {
    return ensurePyodide().then(function (py) {
      var io = collectIo(py);
      return py
        .runPythonAsync(code || "")
        .then(function () {
          var joined = io.join();
          return {
            ok: true,
            stdout: joined.stdout,
            stderr: joined.stderr,
          };
        })
        .catch(function (err) {
          var joined = io.join();
          var message = err && err.message ? err.message : String(err);
          return {
            ok: false,
            stdout: joined.stdout,
            stderr: joined.stderr || message,
            error: message,
          };
        });
    });
  }

  function check(studentCode, testSource) {
    return ensurePyodide().then(function (py) {
      var io = collectIo(py);
      py.FS.writeFile("solution.py", studentCode || "");
      py.FS.writeFile("test_step.py", testSource || "");
      py.FS.writeFile("_check_harness.py", buildCheckHarnessPython());
      return py
        .runPythonAsync(
          "exec(open('_check_harness.py', encoding='utf-8').read(), globals())",
        )
        .then(function () {
          var passed = Boolean(py.runPython("_CHECK_PASSED"));
          var joined = io.join();
          if (passed) {
            return {
              passed: true,
              stdout: joined.stdout,
              stderr: joined.stderr,
              summary: "✓ Checks OK — podés Continuar",
              details: (joined.stdout || "").trim() || "Todos los tests pasaron.",
            };
          }
          return {
            passed: false,
            stdout: joined.stdout,
            stderr: joined.stderr,
            summary: "✗ Checks fallaron — revisá el enunciado y el código",
            details: [joined.stdout, joined.stderr]
              .map(function (s) {
                return (s || "").trim();
              })
              .filter(Boolean)
              .join("\n\n"),
          };
        })
        .catch(function (err) {
          var joined = io.join();
          var message = err && err.message ? err.message : String(err);
          var details = [joined.stdout, joined.stderr, message]
            .map(function (s) {
              return (s || "").trim();
            })
            .filter(Boolean)
            .join("\n\n");
          return {
            passed: false,
            stdout: joined.stdout,
            stderr: joined.stderr || message,
            summary: "✗ Error al validar — revisá la sintaxis o el runtime",
            details: details,
          };
        });
    });
  }

  global.ppiPyodide = {
    version: PYODIDE_VERSION,
    indexURL: PYODIDE_INDEX_URL,
    ensure: ensure,
    run: run,
    check: check,
    formatRunLog: formatRunLog,
    formatCheckLog: formatCheckLog,
    statusMessage: statusMessage,
    isReady: function () {
      return instance !== null;
    },
    getLastError: function () {
      return lastError;
    },
  };
})(typeof window !== "undefined" ? window : globalThis);
