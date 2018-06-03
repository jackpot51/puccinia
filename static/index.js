function generate(response) {
    var transactions = response.transactions;

    chart(document.getElementById("chart_cash_flow"), 'scatter', 'Cash Flow', cash_flow(transactions));
    chart(document.getElementById("chart_net_cash_flow"), 'line', 'Net Cash Flow', net_cash_flow(transactions));
}

function onload() {
    download(function(response) {
        generate(response);
    });
}
