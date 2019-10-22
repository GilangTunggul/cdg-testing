<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Device list</name>
   <tag></tag>
   <elementGuidId>3f12a989-ee34-4a45-85a4-60cd8f6b7fd1</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;function\&quot;: \&quot;list\&quot;,\n    \&quot;query\&quot;: {\n        \&quot;$match\&quot;: {\n            \&quot;name\&quot;: \&quot;\&quot;,\n            \&quot;companyID\&quot;: {\n                \&quot;$oid\&quot;: \&quot;5948f33556d6c91479154da2\&quot;\n            },\n            \&quot;customerID\&quot;: {\n                \&quot;$oid\&quot;: \&quot;${customerID}\&quot;\n            },\n            \&quot;$or\&quot;: [\n                {\n                    \&quot;name\&quot;: {\n                        \&quot;$regex\&quot;: \&quot;\&quot;,\n                        \&quot;$options\&quot;: \&quot;i\&quot;\n                    }\n                }\n            ]\n        },\n        \&quot;$sort\&quot;: [\n            {\n                \&quot;field\&quot;: \&quot;createdAt\&quot;,\n                \&quot;order\&quot;: \&quot;-1\&quot;\n            }\n        ],\n        \&quot;$skip\&quot;: \&quot;0\&quot;,\n        \&quot;$limit\&quot;: \&quot;8\&quot;\n    }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${token}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}api/cdg/customer/products</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>cf32a2cf-edbf-49db-87bd-632ac792cb03</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.tokenapp</defaultValue>
      <description></description>
      <id>3452d19e-8366-4c95-af6e-36e6c6b7769a</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <variables>
      <defaultValue>'5d142a33924e18084fdfd885'</defaultValue>
      <description></description>
      <id>37106bf3-1ed6-4695-ad4d-b6e320baae59</id>
      <masked>false</masked>
      <name>cutomerID</name>
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

println(jsonResponse != null)

WS.verifyElementPropertyValue(response, 'data', jsonResponse.data)

WS.verifyElementPropertyValue(response, 'message', 'success')

def expectedSize = jsonResponse.data.data.size()

println(expectedSize)

for(int i = 0; i &lt; expectedSize; i++) {

	WS.verifyElementPropertyValue(response, 'data.data' + [i] + '.customerID', '5d142a33924e18084fdfd885')
	
	WS.verifyElementPropertyValue(response, 'data.data' + [i] + '.isActive', true)

}

WS.verifyElementPropertyValue(response, 'data.total', jsonResponse.data.total)
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
