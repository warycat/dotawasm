# **[Dota Underlord Perfect Build](https://warycat.github.io/dotawasm)**


This is a wasm web app to help you find optimal build of 10 heroes without wasted alliances in the game [Dota Underlord](https://www.underlords.com/) . Searching all the builds is a NP Hard problem and it took my iMac running on 16 cores for three whole days. 

I build the previous version of the app in SwiftUI on iPhone. I experimented FFI and cargo lipo. It worked but a lot of glue code needed to hook up iOS with the my rust core library. This time I build it in [Seed](https://seed-rs.org/). I really enjoyed the auto formatting and type checking of rust when writing the html part of the app. [Serde-json](https://github.com/serde-rs/json) also works like magic when I export the data from sqlite to json and deserialized the data into rust objects. 

