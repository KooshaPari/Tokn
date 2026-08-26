// Tokn dashboard charts — pure JS (no dependencies)
// Simple canvas-based bar/line chart rendering.

(function () {
  "use strict";

  /**
   * Render a bar chart on a canvas.
   * @param {string} canvasId - ID of the canvas element.
   * @param {Array<{label: string, value: number}>} data - Chart data.
   * @param {Object} [opts] - Optional styling overrides.
   */
  function barChart(canvasId, data, opts) {
    opts = opts || {};
    var canvas = document.getElementById(canvasId);
    if (!canvas) return;
    var ctx = canvas.getContext("2d");
    var w = canvas.width;
    var h = canvas.height;
    var padding = 40;
    var chartW = w - 2 * padding;
    var chartH = h - 2 * padding;
    var max =
      Math.max.apply(
        null,
        data.map(function (d) {
          return d.value;
        }),
      ) || 1;
    var barW = (chartW / data.length) * 0.7;
    var gap = (chartW / data.length) * 0.3;

    ctx.fillStyle = opts.bg || "#0f172a";
    ctx.fillRect(0, 0, w, h);

    ctx.strokeStyle = opts.axis || "#475569";
    ctx.lineWidth = 1;
    ctx.beginPath();
    ctx.moveTo(padding, padding);
    ctx.lineTo(padding, h - padding);
    ctx.lineTo(w - padding, h - padding);
    ctx.stroke();

    ctx.fillStyle = opts.label || "#94a3b8";
    ctx.font = "11px sans-serif";
    ctx.textAlign = "center";
    data.forEach(function (d, i) {
      var barH = (d.value / max) * chartH;
      var x = padding + i * (barW + gap) + gap / 2;
      var y = h - padding - barH;

      ctx.fillStyle = opts.bar || "#38bdf8";
      ctx.fillRect(x, y, barW, barH);

      ctx.fillStyle = opts.label || "#94a3b8";
      ctx.fillText(d.label, x + barW / 2, h - padding + 14);
      ctx.fillText("$" + d.value.toFixed(2), x + barW / 2, y - 4);
    });
  }

  /**
   * Render a horizontal bar chart for top-N lists.
   * @param {string} canvasId - ID of the canvas element.
   * @param {Array<{label: string, value: number}>} data - Chart data.
   */
  function horizontalBar(canvasId, data) {
    var canvas = document.getElementById(canvasId);
    if (!canvas) return;
    var ctx = canvas.getContext("2d");
    var w = canvas.width;
    var h = canvas.height;
    var padding = 40;
    var rowH = (h - 2 * padding) / data.length;
    var max =
      Math.max.apply(
        null,
        data.map(function (d) {
          return d.value;
        }),
      ) || 1;

    ctx.fillStyle = "#0f172a";
    ctx.fillRect(0, 0, w, h);

    data.forEach(function (d, i) {
      var barW = (d.value / max) * (w - 2 * padding - 150);
      var y = padding + i * rowH + 4;

      ctx.fillStyle = "#818cf8";
      ctx.fillRect(padding + 130, y, barW, rowH - 8);

      ctx.fillStyle = "#e2e8f0";
      ctx.font = "12px sans-serif";
      ctx.textAlign = "left";
      ctx.fillText(d.label, padding, y + rowH / 2);

      ctx.textAlign = "right";
      ctx.fillText("$" + d.value.toFixed(2), w - padding, y + rowH / 2);
    });
  }

  // Sample data for the dashboard demo.
  document.addEventListener("DOMContentLoaded", function () {
    var providerCanvas = document.getElementById("provider-chart");
    if (providerCanvas) {
      barChart("provider-chart", [
        { label: "OpenAI", value: 412.5 },
        { label: "Anthropic", value: 267.0 },
        { label: "Google", value: 104.0 },
        { label: "Local", value: 63.71 },
      ]);
    }
    var modelCanvas = document.getElementById("model-chart");
    if (modelCanvas) {
      horizontalBar("model-chart", [
        { label: "gpt-4o", value: 312.0 },
        { label: "claude-3-5-sonnet", value: 174.0 },
        { label: "claude-3-haiku", value: 87.0 },
        { label: "gemini-1.5-pro", value: 76.0 },
        { label: "llama-3-70b-local", value: 63.71 },
      ]);
    }
  });

  // Export for use in other modules.
  window.ToknCharts = { barChart: barChart, horizontalBar: horizontalBar };
})();
