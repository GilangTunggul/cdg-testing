<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Store option</name>
   <tag></tag>
   <elementGuidId>c0b00727-386c-4188-8a1c-e55068f6a0aa</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\t\&quot;function\&quot; : \&quot;datatable\&quot;,\n\t\&quot;query\&quot; : \n\t\t{\n\t\t    \&quot;draw\&quot;: \&quot;1\&quot;,\n\t\t    \&quot;columns\&quot;: [{\n\t\t        \&quot;data\&quot;: \&quot;_id\&quot;,\n\t\t        \&quot;name\&quot;: \&quot;\&quot;,\n\t\t        \&quot;searchable\&quot;: \&quot;true\&quot;,\n\t\t        \&quot;orderable\&quot;: \&quot;true\&quot;,\n\t\t        \&quot;search\&quot;: {\n\t\t            \&quot;value\&quot;: \&quot;\&quot;,\n\t\t            \&quot;regex\&quot;: \&quot;false\&quot;\n\t\t        }\n\t\t    },\n\t\t    {\n\t\t        \&quot;data\&quot;: \&quot;customer.name\&quot;,\n\t\t        \&quot;name\&quot;: \&quot;\&quot;,\n\t\t        \&quot;searchable\&quot;: \&quot;true\&quot;,\n\t\t        \&quot;orderable\&quot;: \&quot;true\&quot;,\n\t\t        \&quot;search\&quot;: {\n\t\t            \&quot;value\&quot;: \&quot;\&quot;,\n\t\t            \&quot;regex\&quot;: \&quot;false\&quot;\n\t\t        }\n\t\t    },\n\t\t    {\n\t\t        \&quot;data\&quot;: \&quot;store.name\&quot;,\n\t\t        \&quot;name\&quot;: \&quot;\&quot;,\n\t\t        \&quot;searchable\&quot;: \&quot;true\&quot;,\n\t\t        \&quot;orderable\&quot;: \&quot;true\&quot;,\n\t\t        \&quot;search\&quot;: {\n\t\t            \&quot;value\&quot;: \&quot;\&quot;,\n\t\t            \&quot;regex\&quot;: \&quot;false\&quot;\n\t\t        }\n\t\t    },\n\t\t    {\n\t\t        \&quot;data\&quot;: \&quot;description\&quot;,\n\t\t        \&quot;name\&quot;: \&quot;\&quot;,\n\t\t        \&quot;searchable\&quot;: \&quot;true\&quot;,\n\t\t        \&quot;orderable\&quot;: \&quot;true\&quot;,\n\t\t        \&quot;search\&quot;: {\n\t\t            \&quot;value\&quot;: \&quot;\&quot;,\n\t\t            \&quot;regex\&quot;: \&quot;false\&quot;\n\t\t        }\n\t\t    },\n\t\t    {\n\t\t        \&quot;data\&quot;: \&quot;status\&quot;,\n\t\t        \&quot;name\&quot;: \&quot;\&quot;,\n\t\t        \&quot;searchable\&quot;: \&quot;true\&quot;,\n\t\t        \&quot;orderable\&quot;: \&quot;true\&quot;,\n\t\t        \&quot;search\&quot;: {\n\t\t            \&quot;value\&quot;: \&quot;\&quot;,\n\t\t            \&quot;regex\&quot;: \&quot;false\&quot;\n\t\t        }\n\t\t    },\n\t\t    {\n\t\t        \&quot;data\&quot;: \&quot;scheduledAt\&quot;,\n\t\t        \&quot;name\&quot;: \&quot;\&quot;,\n\t\t        \&quot;searchable\&quot;: \&quot;true\&quot;,\n\t\t        \&quot;orderable\&quot;: \&quot;true\&quot;,\n\t\t        \&quot;search\&quot;: {\n\t\t            \&quot;value\&quot;: \&quot;\&quot;,\n\t\t            \&quot;regex\&quot;: \&quot;false\&quot;\n\t\t        }\n\t\t    }\n\t\t    ],\n\t\t    \&quot;order\&quot;: [{\n\t\t        \&quot;column\&quot;: 0,\n\t\t        \&quot;dir\&quot;: \&quot;asc\&quot;\n\t\t    }],\n\t\t    \&quot;start\&quot;: 0,\n\t\t    \&quot;length\&quot;: 10,\n\t\t    \&quot;search\&quot;: {\n\t\t        \&quot;value\&quot;: \&quot;\&quot;,\n\t\t        \&quot;regex\&quot;: \&quot;false\&quot;\n\t\t    }\n\t\t}\n}&quot;,
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
      <name>Cookie</name>
      <type>Main</type>
      <value>${cookies}</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url}module/cdg/main/customer/store/selection?</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>31b3cca4-51e7-4cdd-bc74-8f248dc6a62a</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cookies</defaultValue>
      <description></description>
      <id>f59963a5-1a5a-4751-9778-2f98241484c8</id>
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

assertThat(response.getHeaderFields().get('Content-Type').toString()).isEqualTo('[text/html; charset=UTF-8]')

assertThat(response.getHeaderFields().containsKey('Content-Type')).isTrue()

def jsonSlurper = new JsonSlurper()

def jsonResponse = jsonSlurper.parseText(response.getResponseText())

println(jsonResponse)

if(jsonResponse.data == null){
	
	WS.verifyElementPropertyValue(response, 'data', '[]')
	
	WS.verifyElementPropertyValue(response, 'success', 'true')
	
} else {
	
    WS.verifyElementPropertyValue(response, 'success', 'true')

}


</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
