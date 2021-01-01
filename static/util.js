function child(parent, tag) {
    return parent.appendChild(document.createElement(tag));
}

function create_checkbox_filter(key) {
    return function(item) {
        var element = document.getElementById('checkbox_' + key + '_' + item[key]);
        return element && element.checked;
    }
}

function create_time_filter(since_id, until_id, nested_filter = null) {
    var since = document.getElementById(since_id).value;
    var until = document.getElementById(until_id).value;
    return function(item) {
        if (nested_filter && !nested_filter(item)) {
            return false;
        }

        if (since && since > item.time) {
            return false;
        }

        if (until && until < item.time) {
            return false;
        }

        return true;
    };
}

function download(callback) {
    var xhttp = new XMLHttpRequest();
    xhttp.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            var response = JSON.parse(this.responseText);
            callback(response);
        }
    };
    xhttp.open("GET", "/json", true);
    xhttp.send();
}

function chart_divs(element, ids) {
    var deck = null;
    for (var i in ids) {
        if (i % 2 == 0) {
            deck = child(element, "div");
            deck.classList.add("card-deck");
        }

        var card = child(deck, "div");
        card.classList.add("card");
        card.style.marginTop = "15px";
        card.style.marginBottom = "15px";

        var card_body = child(card, "div");
        card_body.classList.add("card-body");

        var canvas = child(card_body, "canvas");
        canvas.id = ids[i];
        canvas.setAttribute("width", 300);
        canvas.setAttribute("height", 300);
    }
}

function chart(element, type, title, data, fullscreen = false) {
    if (element.chart) {
        element.chart.destroy();
        delete element.chart;
    }

    var maintainAspectRatio = true;
    if (fullscreen) {
        element.onclick = function() {
            var table_window = window.open("/static/table.html");
            table_window.table_data = {
                "title": title,
                "data": data
            };
        };
        maintainAspectRatio = false;
    } else {
        element.onclick = function() {
            var chart_window = window.open("/static/chart.html");
            chart_window.chart_data = {
                "type": type,
                "title": title,
                "data": data
            };
        };
    }

    Chart.defaults.global.defaultFontColor = '#ffffff';
    element.chart = new Chart(element.getContext('2d'), {
        type: type,
        data: {
            datasets: [{
                label: title,
                data: data.slice(),
                backgroundColor: "rgba(255, 255, 255, 0.1)",
                borderColor: "rgba(255, 255, 255, 0.1)",
            }]
        },
        options: {
            responsive: true,
            maintainAspectRatio: maintainAspectRatio,
            scales: {
                xAxes: [{
                    type: 'time',
                    display: true,
                    scaleLabel: {
                        display: true,
                        labelString: 'Date'
                    }
                }]
            },
            tooltips: {
                mode: 'index',
                callbacks: {
                    title: function(tooltipItems, data) {
                        var tooltipItem = tooltipItems[0];
                        var item = data.datasets[tooltipItem.datasetIndex].data[tooltipItem.index];
                        return item.title;
                    },
                    label: function(tooltipItem, data) {
                        var item = data.datasets[tooltipItem.datasetIndex].data[tooltipItem.index];
                        return item.label;
                    },
                    afterLabel: function(tooltipItem, data) {
                        var item = data.datasets[tooltipItem.datasetIndex].data[tooltipItem.index];
                        return item.y;
                    }
                },
                footerFontStyle: 'normal'
            }
        }
    });

    return element.chart;
}

function convert_transactions(transactions, position_transactions) {
    transactions.forEach(function(transaction) {
        position_transactions.push({
            wallet_id: transaction.wallet_id,
            account_id: transaction.account_id,
            position_id: "balance",
            id: transaction.id,
            name: transaction.name,
            time: transaction.time,
            units: transaction.amount,
            price: "1",
            value: (parseFloat(transaction.amount)).toString()
        });
    });
}

function change_in_value(positions, transactions, prices) {
    var value_data = value(positions, transactions, prices);

    var data = [];
    for (var i = 1; i < value_data.length; i++) {
        var last = value_data[i - 1];
        var point = value_data[i];

        data.push({
            x: point.x,
            y: point.y - last.y,
            title: point.title,
            label: point.label
        });
    }

    return data;
}

function value(positions, transactions, prices) {
    var data = [];
    var last_y = {};
    positions.forEach(function(position) {
        var position_path = position.wallet_id + "/" + position.account_id + "/" + position.id;

        var position_transactions = transactions.filter(function(transaction) {
            return transaction.wallet_id == position.wallet_id
                && transaction.account_id == position.account_id
                && transaction.position_id == position.id;
        });

        var position_prices = prices.filter(function(price) {
            return price.wallet_id == position.wallet_id
                && price.account_id == position.account_id
                && price.position_id == position.id;
        });

        var share_data = share_value(position, position_transactions, position_prices);

        var last_time = 0;
        for (var i = share_data.length - 1; i >= 0; i--) {
            var share_point = share_data[i];

            if (i == 0) {
                last_y[position_path] = share_point.y;
            }

            if (share_point.x.getTime() == last_time) {
                continue;
            }

            var point = data.find(function(point) {
                return point.x.getTime() == share_point.x.getTime();
            });

            if (point === undefined) {
                var y = {};
                y[position_path] = parseFloat(share_point.y);
                data.push({
                    x: share_point.x,
                    y: y,
                    title: moment(share_point.x).format("YYYY-MM-DD"),
                    label: positions.length + " position"  + (positions.length === 1 ? "" : "s")
                });
            } else {
                point.y[position_path] = share_point.y;
            }

            last_time = share_point.x.getTime();
        }
    });

    data.sort(function(a, b) {
        return a.x.getTime() - b.x.getTime();
    });

    for (var i = 0; i < data.length; i++) {
        var point = data[i];

        var total = 0;
        positions.forEach(function(position) {
            var position_path = position.wallet_id + "/" + position.account_id + "/" + position.id;
            if (point.y.hasOwnProperty(position_path)) {
                last_y[position_path] = point.y[position_path];
            }
            if (!last_y.hasOwnProperty(position_path)) {
                last_y[position_path] = 0.0;
            }
            total += last_y[position_path];
        });
        point.y = total;
    }

    return data;
}

function share_price(prices) {
    var data = [];
    for (var i = 0; i < prices.length; i++) {
        var price = prices[i];

        var date = new Date(price.time);
        var value = parseFloat(price.price);
        data.push({
            x: date,
            y: Math.round(value * 100.0)/100.0,
            title: price.time,
            label: price.position_id
        });
    }
    return data;
}

function share_value(position, transactions, prices) {
    var data = [];
    var current_units = parseFloat(position.units);

    //TODO: Record position update time
    var today = new Date();
    today.setUTCHours(0, 0, 0, 0);
    data.push({
        x: today,
        y: Math.round(parseFloat(position.price) * current_units * 100.0)/100.0,
        title: "Current Price",
        label: position.id + ": " + current_units + " @ " + position.price
    });

    var add_price = function(price) {
        data.push({
            x: new Date(price.time),
            y: Math.round(parseFloat(price.price) * current_units * 100.0)/100.0,
            title: price.time + " Price",
            label: price.position_id + ": " + current_units + " @ " + price.price
        });
    };

    var j = prices.length - 1;
    for (var i = transactions.length - 1; i >= 0; i--) {
        var transaction = transactions[i];

        for(; j >= 0; j--) {
            var price = prices[j];

            if (price.time <= transaction.time) {
                break;
            }

            add_price(price);
        }

        data.push({
            x: new Date(transaction.time),
            y: Math.round(parseFloat(transaction.price) * current_units * 100.0)/100.0,
            title: transaction.time + " Transaction",
            label: transaction.position_id + ": " + current_units + " @ " + transaction.price
        });

        for(; j >= 0; j--) {
            var price = prices[j];

            if (price.time < transaction.time) {
                break;
            }

            add_price(price);
        }

        current_units -= parseFloat(transaction.units);
    }

    for (; j >= 0; j--) {
        var price = prices[j];

        add_price(price);
    }

    return data.reverse();
}

function cash_flow(transactions, amount_key="amount") {
    var data = [];
    for (var i = 0; i < transactions.length; i++) {
        var transaction = transactions[i];

        var date = new Date(transaction.time);
        var amount = parseFloat(transaction[amount_key]);
        data.push({
            x: date,
            y: Math.round(amount * 100.0)/100.0,
            title: transaction.time,
            label: transaction.name
        });
    }

    return data;
}

function net_cash_flow(transactions, amount_key="amount") {
    var data = [];
    var total = 0.0;
    for (var i = 0; i < transactions.length; i++) {
        var transaction = transactions[i];

        var date = new Date(transaction.time);
        var amount = parseFloat(transaction[amount_key]);
        total += amount;
        data.push({
            x: date,
            y: Math.round(total * 100.0)/100.0,
            title: transaction.time,
            label: transaction.name
        });
    }

    return data;
}

function expenses(transactions, transfers) {
    var data = [];
    for (var i = 0; i < transactions.length; i++) {
        var transaction = transactions[i];

        var transfer = transfers.find(function(transfer) {
            return (
                transaction.wallet_id == transfer.from_wallet_id &&
                transaction.account_id == transfer.from_account_id &&
                transaction.id == transfer.from_id
            );
        });
        if (transfer === undefined) {
            var date = new Date(transaction.time);
            var amount = parseFloat(transaction.amount);
            if (amount < 0) {
                data.push({
                    x: date,
                    y: Math.round(amount * 100.0)/100.0,
                    title: transaction.time,
                    label: transaction.name
                });
            }
        }
    }

    return data;
}

function net_expenses(transactions, transfers) {
    var data = [];
    var total = 0.0;
    for (var i = 0; i < transactions.length; i++) {
        var transaction = transactions[i];

        var transfer = transfers.find(function(transfer) {
            return (
                transaction.wallet_id == transfer.from_wallet_id &&
                transaction.account_id == transfer.from_account_id &&
                transaction.id == transfer.from_id
            );
        });
        if (transfer === undefined) {
            var date = new Date(transaction.time);
            var amount = parseFloat(transaction.amount);
            if (amount < 0) {
                total += amount;
                data.push({
                    x: date,
                    y: Math.round(total * 100.0)/100.0,
                    title: transaction.time,
                    label: transaction.name
                });
            }
        }
    }

    return data;
}

function income(transactions, transfers) {
    var data = [];
    for (var i = 0; i < transactions.length; i++) {
        var transaction = transactions[i];

        var transfer = transfers.find(function(transfer) {
            return (
                transaction.wallet_id == transfer.to_wallet_id &&
                transaction.account_id == transfer.to_account_id &&
                transaction.id == transfer.to_id
            );
        });
        if (transfer === undefined) {
            var date = new Date(transaction.time);
            var amount = parseFloat(transaction.amount);
            if (amount > 0) {
                data.push({
                    x: date,
                    y: Math.round(amount * 100.0)/100.0,
                    title: transaction.time,
                    label: transaction.name
                });
            }
        }
    }

    return data;
}

function net_income(transactions, transfers) {
    var data = [];
    var total = 0.0;
    for (var i = 0; i < transactions.length; i++) {
        var transaction = transactions[i];

        var transfer = transfers.find(function(transfer) {
            return (
                transaction.wallet_id == transfer.to_wallet_id &&
                transaction.account_id == transfer.to_account_id &&
                transaction.id == transfer.to_id
            );
        });
        if (transfer === undefined) {
            var date = new Date(transaction.time);
            var amount = parseFloat(transaction.amount);
            if (amount > 0) {
                total += amount;
                data.push({
                    x: date,
                    y: Math.round(total * 100.0)/100.0,
                    title: transaction.time,
                    label: transaction.name
                });
            }
        }
    }

    return data;
}
