use reqwest;
use scraper::{Html, Selector};
use tokio;

struct Job {
    url: String,
    items: Vec<JobOutputItem>,
}

struct JobOutputItem {
    id: String,
    selector: String,
}

struct JobOutputItemValue {
    id: String,
    value: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let job = Job {
        url: "https://finance.yahoo.com/quote/GOOG/analysis".to_string(),
        items: vec![
            JobOutputItem {
                id: "name".to_string(),
                selector: "h1".to_string(),
            },
            JobOutputItem {
                id: "price".to_string(),
                selector: r#"#quote-header-info > div.My\(6px\).Pos\(r\).smartphone_Mt\(6px\) > div.D\(ib\).Va\(m\).Maw\(65\%\).Ov\(h\) > div > span.Trsdu\(0\.3s\).Fw\(b\).Fz\(36px\).Mb\(-4px\).D\(ib\)"#.to_string(),
            },
            JobOutputItem {
                id: "earnings".to_string(),
                selector: "#Col1-0-AnalystLeafPage-Proxy > section > table:nth-child(2) > tbody > tr:nth-child(2) > td:nth-child(4) > span".to_string(),
            },
            JobOutputItem {
                id: "revenue".to_string(),
                selector: "#Col1-0-AnalystLeafPage-Proxy > section > table:nth-child(3) > tbody > tr:nth-child(2) > td:nth-child(4) > span".to_string(),
            },
            JobOutputItem {
                id: "sales_growth".to_string(),
                selector: "#Col1-0-AnalystLeafPage-Proxy > section > table:nth-child(3) > tbody > tr:nth-child(6) > td:nth-child(4) > span".to_string(),
            },
        ],
    };

    let body = reqwest::get(&job.url).await?.text().await?;

    println!("body = {:?}...", &body[..60]);

    let result = extract_values(body, &job.items);

    for item in result {
        println!("{}: {}", item.id, item.value);
    }

    Ok(())
}

fn extract_values(html: String, items: &Vec<JobOutputItem>) -> Vec<JobOutputItemValue> {
    let fragment = Html::parse_fragment(&html);

    items
        .iter()
        .map(|item| JobOutputItemValue {
            id: item.id.to_string(),
            value: extract_value(&item.selector, &fragment),
        })
        .collect()
}

fn extract_value(selector: &str, fragment: &Html) -> String {
    let selector = Selector::parse(selector).unwrap();

    let h1 = fragment.select(&selector).next().unwrap();
    // let text = h1.text().collect::<Vec<_>>();
    let text = h1.inner_html();

    text
}
