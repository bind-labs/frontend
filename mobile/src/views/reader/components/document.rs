use dioxus::prelude::*;

#[component]
pub fn ReaderDocument() -> Element {
    rsx! {
        div { class: "reader-document",
            p {
                "Tomorrow, 26 February, "
                a { href: "https://www.spacex.com/", target: "_blank", "SpaceX" }
                " will launch a Falcon 9 rocket carrying an Intuitive Machines mission that will stay on the surface of the moon for approximately three weeks before returning to Earth. Among other  things, the Intuitive Machines lander contains a mini data center,  massing just 1 kilogram and containing 8 terabytes of SSD storage. This belongs to Lonestar Data Holdings and is part of a proof-of-concept mission meant to bring moon-based data centers closer to reality."
            }
            p {
                "The idea of putting a data center on the moon raises a natural question: Why? Lonestar’s CEO Christopher Stott says it is to protect sensitive data froThemem Earthly hazards."
            }
            blockquote {
                "Data centers, right? They’re like modern cathedrals. We’re building these  things, they run our entire civilization. It’s superb, and yet you  realize that the networks connecting them are increasingly fragile."
            }
            h3 { "The Case for Moon-based Data Centers" }
            p {
                "Indeed, on Earth, undersea cables often get cut, leading to outages. Natural disasters like hurricanes and earthquakes, as well as war, can also disrupt networks or destroy the data itself.  The lunar surface is a much more predictable place—there is almost no  atmosphere, and therefore no climate events to worry about. There is radiation, but it is fairly constant. And the moon is not a war zone, at least for now."
            }
            p {
                "“We call it resilience as a service,” Stott says. “It’s like a whole new level of backup that we’ve never had before.”"
            }
            p {
                "The other motivation is data sovereignty. Over 100 countries worldwide have laws that restrict where certain data can be processed and stored, often to within that country itself. As a  data center provider, it’s impossible to accommodate all potential  customers in any one location, except in outer space. According to the  United Nations’ 1967 outer space treaty, space and the moon are “not subject to national appropriation by claim  of sovereignty,” and as such poses a loophole for data sovereignty laws. An American satellite is under American law, but it can carry a black  box inside it that’s under British law, or any other country’s. A  moon-based data center can host as many separate black boxes as needed, to accommodate all of its diverse customers."
            }
            p {
                "Governments seem particularly interested in this prospect. This test mission will  carry data for the Florida state government as well as for the Isle of  Man. They will also carry a copy of Bethesda Games’Starfield, and will be transmitting the game’s featured song “Children of the Sky” by Imagine Dragons back to Earth throughout the mission, just for fun."
            }
            p {
                "Amit Verma, a professor of electrical engineering at Texas A&M University Kingsville who is not affiliated with the  project, says there may be technical advantages to hosting data on the  moon as well. Some parts of the moon are permanently shadowed and  therefore extremely cold, as low as -173 °C. This means that no energy or water would need to be expended to cool  the data center. And the electrical components will perform more  efficiently."
            }
            p {
                "“When you place data centers in environments that are already very, very cold...the performance actually also improves  significantly,” Verma says. “Because when you go down in temperature,  things like electrical resistance also go down.”"
            }
            p {
                "Future moon-based data centers could be powered entirely through solar, since the parts  of the moon’s surface that are always cold, near the lunar poles, are  relatively close to crater rims that are nearly always exposed to  sunlight, unattenuated by an atmosphere.  Theoretically, data centers  can be hidden away from the sun and power can be transmitted from these  rims, resulting in perfectly renewable operation at low temperature."
            }
        }
    }
}

#[component]
pub fn ReaderDocumentHtml() -> Element {
    rsx! {
        div { class: "reader-document-html",
            p {
                "Tomorrow, 26 February, SpaceX will launch a Falcon 9 rocket carrying an Intuitive Machines mission that will stay on the surface of the moon for approximately three weeks before returning to Earth. Among other  things, the Intuitive Machines lander contains a mini data center,  massing just 1 kilogram and containing 8 terabytes of SSD storage. This belongs to Lonestar Data Holdings and is part of a proof-of-concept mission meant to bring moon-based data centers closer to reality."
            }
            p {
                "The idea of putting a data center on the moon raises a natural question: Why? Lonestar’s CEO Christopher Stott says it is to protect sensitive data froThemem Earthly hazards."
            }
            p {
                "“Data centers, right? They’re like modern cathedrals. We’re building these  things, they run our entire civilization. It’s superb, and yet you  realize that the networks connecting them are increasingly fragile.”"
            }
            h3 { "The Case for Moon-based Data Centers" }
            p {
                "Indeed, on Earth, undersea cables often get cut, leading to outages. Natural disasters like hurricanes and earthquakes, as well as war, can also disrupt networks or destroy the data itself.  The lunar surface is a much more predictable place—there is almost no  atmosphere, and therefore no climate events to worry about. There is radiation, but it is fairly constant. And the moon is not a war zone, at least for now."
            }
            p {
                "“We call it resilience as a service,” Stott says. “It’s like a whole new level of backup that we’ve never had before.”"
            }
            p {
                "The other motivation is data sovereignty. Over 100 countries worldwide have laws that restrict where certain data can be processed and stored, often to within that country itself. As a  data center provider, it’s impossible to accommodate all potential  customers in any one location, except in outer space. According to the  United Nations’ 1967 outer space treaty, space and the moon are “not subject to national appropriation by claim  of sovereignty,” and as such poses a loophole for data sovereignty laws. An American satellite is under American law, but it can carry a black  box inside it that’s under British law, or any other country’s. A  moon-based data center can host as many separate black boxes as needed, to accommodate all of its diverse customers."
            }
            p {
                "Governments seem particularly interested in this prospect. This test mission will  carry data for the Florida state government as well as for the Isle of  Man. They will also carry a copy of Bethesda Games’Starfield, and will be transmitting the game’s featured song “Children of the Sky” by Imagine Dragons back to Earth throughout the mission, just for fun."
            }
            p {
                "Amit Verma, a professor of electrical engineering at Texas A&M University Kingsville who is not affiliated with the  project, says there may be technical advantages to hosting data on the  moon as well. Some parts of the moon are permanently shadowed and  therefore extremely cold, as low as -173 °C. This means that no energy or water would need to be expended to cool  the data center. And the electrical components will perform more  efficiently."
            }
            p {
                "“When you place data centers in environments that are already very, very cold...the performance actually also improves  significantly,” Verma says. “Because when you go down in temperature,  things like electrical resistance also go down.”"
            }
            p {
                "Future moon-based data centers could be powered entirely through solar, since the parts  of the moon’s surface that are always cold, near the lunar poles, are  relatively close to crater rims that are nearly always exposed to  sunlight, unattenuated by an atmosphere.  Theoretically, data centers  can be hidden away from the sun and power can be transmitted from these  rims, resulting in perfectly renewable operation at low temperature."
            }
    }
    }
}
