import './css/index.css';
import './css/index.scss';

import $ from "jquery";
import template from 'lodash/template';

const outputElement = document.getElementById('app');
if (outputElement) {
  var compiled = template(`
    <h1 id="h1-test"><%- heading %></h1>
    Current date and time: <%- dateTimeString %>
  `.trim());
  outputElement.innerHTML = compiled({
    heading: 'ts-demo-webpack',
    dateTimeString: new Date().toISOString(),
  });

  $('#h1-test').click(()=>{
    alert('123456');
  });
}
