<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Read All Notification</name>
   <tag></tag>
   <elementGuidId>b956a12d-6e8d-4252-a138-25dd524d783c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;contentType&quot;: &quot;multipart/form-data&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;,
  &quot;parameters&quot;: [
    {
      &quot;name&quot;: &quot;companyID&quot;,
      &quot;value&quot;: &quot;${companyID}&quot;,
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
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${url}/module/cdg/customer/${customerID}/notification/read?companyID=${companyID}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>5d064102-24f1-47b6-8ee0-fbc3586b2cbf</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.cookies</defaultValue>
      <description></description>
      <id>613f398b-f586-4fdc-8616-13fcda7728a1</id>
      <masked>false</masked>
      <name>cookies</name>
   </variables>
   <variables>
      <defaultValue>'5d142a33924e18084fdfd885'</defaultValue>
      <description></description>
      <id>23da7e5e-b1b7-4775-a874-5efaa5bb173c</id>
      <masked>false</masked>
      <name>customerID</name>
   </variables>
   <variables>
      <defaultValue>'5948f33556d6c91479154da2'</defaultValue>
      <description></description>
      <id>3287f736-7c1a-4912-b80d-4357266c4b3c</id>
      <masked>false</masked>
      <name>companyID</name>
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

WS.verifyElementPropertyValue(response, 'message', '[Success]')

WS.verifyElementPropertyValue(response, 'data', '[]')

WS.verifyElementPropertyValue(response, 'success', 'true')
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
