# uWthr

uWthr is a tiny CLI app to get the current weather. It was main to be small, quick, and to the point. Weather APIs are somewhat limited, this app uses openweathermap as it is free. 

### Usage

Fetch the weather at your current location

```shell
uwthr

Weather: Denver, US
               few clouds
     .__.      88°f - feels like 81°f
  ._(    ).    H/L - 90°f / 85°f
 (___.__)__)   Wind - 5mph
               Humidity - 11%
```

Fetch the weather at a location by string. More detailed searches or searches with spaces must be included in quotation marks. Quotation mark format is city,state,country for usa and city,country for global.

examples
    - copenhagen
    - "los angeles"
    - "austin,tx,usa"

```shell
uwthr copenhagen

Weather: Copenhagen, DK
     .__.      light rain
  ._(    ).    61°f - feels like 56°f
 (___.__)__)   H/L - 61°f / 60°f
   ʻ  ʻ  ʻ     Wind - 14mph
    ʻ  ʻ       Humidity - 93%
```

Fetch the weather at a location by zip code. (USA only) Supply a 6 digit zip code. Keep in mind that Openweathermap uses its own zip code mapping.

```shell
uwthr 90001

Weather: Los Angeles, US
    \   /      clear sky
     .-.       72°f - feels like 71°f
  ~ (   ) ~    H/L - 80°f / 69°f
     `-᾿       Wind - 6mph
    /   \      Humidity - 60%
```

Include the ```-c``` or ```-k``` flags to set the units.

### Setup

To use uwthr, you need an openweathermap api code. You can create a free account. To set the api code use the following command. 
```
uwthr -set apicode
```

### Installation

requires rust

```shell
git clone git@gitlab.com:lostVkng/uwthr.git
cargo build --release
```

Copy the compiled binary (in the target directory) to your preferred location. Ex: ```/usr/local/bin``` Add to path if needed. 

### Credits

Enjoy! 