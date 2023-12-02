use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Location {
    key: String,
    localized_name: String,
    country: Country,
}
impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.localized_name, self.country.id)
    }
}

#[derive(Deserialize, Debug)]
pub struct Country {
    #[serde(alias = "ID")]
    pub id: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Forecast {
    pub headline: Headline,
}

#[derive(Deserialize, Debug)]
pub struct Headline {
    #[serde(alias = "Text")]
    pub overview: String,
}

#[derive(Debug)]
pub struct CouldNotFindLocation {
    place: String,
}

impl Display for CouldNotFindLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not find location '{}'", self.place)
    }
}

impl std::error::Error for CouldNotFindLocation {}

pub async fn get_forcast(
    place: &str,
    api_key: &str,
    client: Client,
) -> Result<(Location, Forecast), Box<dyn std::error::Error>> {
    // API endpoints
    const LOCATION_REQUEST: &str = "http://dataservice.accuweather.com/locations/v1/cities/search";
    const DAY_REQUEST: &str = "http://dataservice.accuweather.com/forecasts/v1/daily/1day/";

    // Request URL
    let url = format!("{}?apikey={}&q={}", LOCATION_REQUEST, api_key, place);
    // Build the request
    let request = client.get(url).build().unwrap();

    // Execute the request
    let resp = client
        .execute(request)
        .await?
        .json::<Vec<Location>>()
        .await?;

    // Get the first location, err if not found
    let first_location = resp
        .into_iter()
        .next()
        .ok_or_else(|| CouldNotFindLocation {
            place: place.to_owned(),
        })?;
    
    // Combine the location with the URL (and API key)
    let url = format!("{}{}?apikey={}", DAY_REQUEST, first_location.key, api_key);

    let request = client.get(url).build().unwrap();
    let forecast = client
        .execute(request)
        .await?
        .json::<Forecast>()
        .await?;

    // Return the location and forecast
    Ok((first_location, forecast))
}
