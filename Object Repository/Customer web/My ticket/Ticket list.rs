<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Ticket list</name>
   <tag></tag>
   <elementGuidId>bdc820af-7279-41e2-ba5e-781d5fe4151d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;function&quot;,
      &quot;value&quot;: &quot;list&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[$match][$or][0][productName][$regex]&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[$match][$or][0][productName][$options]&quot;,
      &quot;value&quot;: &quot;i&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[$match][$or][1][ticketNumber][$regex]&quot;,
      &quot;value&quot;: &quot;&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[$match][$or][1][ticketNumber][$options]&quot;,
      &quot;value&quot;: &quot;i&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[$sort][0][field]&quot;,
      &quot;value&quot;: &quot;createdAt&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[$sort][0][order]&quot;,
      &quot;value&quot;: &quot;-1&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[$skip]&quot;,
      &quot;value&quot;: &quot;0&quot;,
      &quot;type&quot;: &quot;Text&quot;
    },
    {
      &quot;name&quot;: &quot;query[$limit]&quot;,
      &quot;value&quot;: &quot;8&quot;,
      &quot;type&quot;: &quot;Text&quot;
    }
  ]
}</httpBodyContent>
   <httpBodyType>form-data</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>multipart/form-data</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Cookie</name>
      <type>Main</type>
      <value>${cookies}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}module/cdg/main/customer/ticket/list</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>13375815-9404-407b-96b9-7bd1860c898a</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cookies</defaultValue>
      <description></description>
      <id>e2ef54f8-b980-4f7b-b4ba-0a2a68f6e9c3</id>
      <masked>false</masked>
      <name>cookies</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

def jsonSlurper = new JsonSlurper()

def jsonResponse = jsonSlurper.parseText(response.getResponseText())

JsonSlurper slurper = new JsonSlurper()

def parsedJson = slurper.parseText(response.getResponseText())

println(jsonResponse.data)

if (jsonResponse.data == [] ) {
	
	WS.verifyElementPropertyValue(response, 'success', 'true')
	
	WS.verifyElementPropertyValue(response, 'data', '[]')
	
} else {
	
	WS.verifyElementPropertyValue(response, 'message', '[]')

	WS.verifyElementPropertyValue(response, 'data', jsonResponse.data)
	
	WS.verifyElementPropertyValue(response, 'success', 'true')
	
	def expectedSize = jsonResponse.data.data.size()
	
	println(expectedSize)
	
	for(int i = 0; i &lt; expectedSize; i++) {
	
		WS.verifyElementPropertyValue(response, 'data.data' + [i] + '.customerID', '5d142a33924e18084fdfd885')
		
		WS.verifyElementPropertyValue(response, 'data.data' + [i] + '.companyID', '5948f33556d6c91479154da2')
	
	}
	
	String a = parsedJson.data.data.id
	
	String b = parsedJson.data.data.productNumber
	
	String c = parsedJson.data.data.ticketNumber
	
	String[] arrayResponse = [a, b, c]
	
	assertThat(arrayResponse).containsOnly(a, b, c)
	
	assertThat(arrayResponse).containsOnlyElementsOf(Arrays.asList(a, b, c))
	
}
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
