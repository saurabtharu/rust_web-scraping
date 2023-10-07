

```rust
let document = scraper::Html::parse_document(&page);


// getting all the elements with class "athing"
let athing_selector = scraper::Selector::parse(".athing").unwrap();
let athing_elements = document.select(&athing_selector);            // select all the elements from `document` for which athing_selector matches

/*



<tr class="athing" id="37800951">
  <td align="right" valign="top" class="title"><span class="rank">1.</span></td>
  <td valign="top" class="votelinks">
    <center>
      <a id="up_37800951" href="vote?id=37800951&amp;how=up&amp;goto=news"
        ><div class="votearrow" title="upvote"></div
      ></a>
    </center>
  </td>
  <td class="title">
    <span class="titleline"
      ><a
        href="https://nerdyarticles.com/a-clutter-free-life-with-paperless-ngx/"
        rel="noreferrer"
        >A Clutter-Free Life: Going Paperless with Paperless-Ngx</a
      ><span class="sitebit comhead">
        (<a href="from?site=nerdyarticles.com"
          ><span class="sitestr">nerdyarticles.com</span></a
        >)</span
      ></span
    >
  </td>
</tr>
<tr>
  <td colspan="2"></td>
  <td class="subtext">
    <span class="subline">
      <span class="score" id="score_37800951">40 points</span> by
      <a href="user?id=thunderbong" class="hnuser">thunderbong</a>
      <span class="age" title="2023-10-07T11:55:09" >
        <a href="item?id=37800951">57 minutes ago</a>
        </span >
      <span id="unv_37800951"></span> |
      <a href="hide?id=37800951&amp;goto=news">hide</a> |
      <a href="item?id=37800951">14&nbsp;comments</a>
    </span>
  </td>
</tr>
<tr class="spacer" style="height: 5px"></tr>


*/


// now iterate over athing_elements 
for athing_element in athing_elements {
    // here contnet we want is in `anchor` tag which is inside the `span` tag with classname "titleline"
    let titleline_selector = scraper::Selector::parse(".titleline").unwrap();
    let titleline_element = athing_element.select(&titleline_selector);

    let anchor_selector = scraper::Selector::parse("a").unwrap();
    let anchor_element = titleline_element.select(&anchor_selector).next().unwrap(); 

    // to retrieve the text present inside the anchor tag
    let heading = anchor_element.text().collect::<String>();
    // to retrieve the value inside the `href` attribute of the anchor tag
    let link = anchor_element.value().attr("href").unwrap().to_string();


    // next sibling of element of tag with class "athing" 
    let athing_next_sibling = athing_element.next_sibling().unwrap();
    let athing_next_sibling_element = ElementRef::wrap(athing_next_sibling).unwrap();

    // now we can get the age and score as well as author from the athing_next_sibling
    let age_selector = scraper::Selector::parse(".age").unwrap();
    let age_element = athing_next_sibling_element.select(&age_selector).next().unwrap();
    // now retrieve the text present inside the `age_element`
    let age = age_element.text().collect::<String>();


    // now get the element of with class name "score"
    let score_selector = scraper::Selector::parse(".score").unwrap();
    let score_element = athing_next_sibling_element.select(&score_selector).next().unwrap();
    // now retrieve the text present inside the `score_element`
    let score = score_element.text().collect::<String()>;


}



```