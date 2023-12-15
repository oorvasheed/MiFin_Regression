<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>body_Code added by Naveen Baghel10122015set_bd916e</name>
   <tag></tag>
   <elementGuidId>bb020423-65aa-4f1a-8954-428eb72aac04</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>body.menuHavingBody</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//body</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>body</value>
      <webElementGuid>2ec945e6-ba98-4312-98e8-ab91c9843301</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>onload</name>
      <type>Main</type>
      <value>populate()</value>
      <webElementGuid>0a4d1e2d-2395-402f-a2d8-8a66de9f4c55</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>onkeydown</name>
      <type>Main</type>
      <value>showKeyCode(event);</value>
      <webElementGuid>f2fc7da7-0f99-406e-93bf-1a032b48614a</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>topmargin</name>
      <type>Main</type>
      <value>0</value>
      <webElementGuid>4fc0d01b-6405-4e7a-8da4-07698af9e991</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>leftmargin</name>
      <type>Main</type>
      <value>0</value>
      <webElementGuid>361d4dec-6d62-4fb7-b3fe-3249a8349fcb</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>menuHavingBody</value>
      <webElementGuid>e2661124-0620-445c-ad6a-0758317ea004</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
	
			


 
 
 
	
	
	
	
	
	
	 







//Code added by Naveen Baghel::10/12/2015


setTimeout(&quot;setToolTip();&quot;,100);

function setTitleforVarFields()
{
	for(count=0; count &lt; document.forms[0].elements.length; count+=1)
    {
    	theelement = document.forms[0].elements[count];
    	if(theelement)
        {
	        if(theelement.type == &quot;text&quot;)
	        {
	        	theelement.title = (theelement.value).toUpperCase();
	        	
	        }
	      
	        
	     }
    }   
}


   function setToolTip()

   {try{

   if(document.forms[0])

   {

    for(count=0; count &lt; document.forms[0].elements.length; count+=1)

        {

          theelement = document.forms[0].elements[count];

                   if(theelement.options != null)

                   {
                     showTooltip(theelement);
                   }

                   else

                   {

                    if(theelement.type!=&quot;checkbox&quot; &amp;&amp; theelement.type!=&quot;radio&quot; &amp;&amp;  theelement.value!=null &amp;&amp; theelement.value!=&quot;&quot; )

                     theelement.title=theelement.value;  
                     changeToolTipText(theelement);     
                   }

        }
    }

    }catch(err){

    //alert(err);

    }

    }
    
    function setToolTipText(theelement)
    {
       theelement.title=theelement.value;   
    }

function showTooltip(theelement)

                        {    
                                                                                             

            for(i=0; i&lt;theelement.options.length; i++)

               {

                 theelement.options[i].title=theelement.options[i].text;

               }

               if(theelement.addEventListener){

                                                  theelement.addEventListener(&quot;change&quot;, function(e){

                                                    e = e || event;

                                                    changeToolTip();

                                                  }, false);
                                                 

                                                }

                                                else if(theelement.attachEvent){

                                                  theelement.attachEvent(&quot;onchange&quot;, function(e){

                                                    e = e || event;

                                                    changeToolTip();

                                                  });
                                                  
                                                }

                                                try

                                                {

                if(parseInt(theelement.options.length)!=0)

               theelement.title=theelement.options[theelement.selectedIndex].text;

               else

               theelement.title=theelement.value;

               }

               catch(err)

               {

                        theelement.title=&quot;&quot;;

               }

                        }

        
   function changeToolTipText(theelement)
   {
       if(theelement.addEventListener){

                                                  theelement.addEventListener(&quot;keyup&quot;, function(e){

                                                    e = e || event;

                                                    setToolTipText(theelement);

                                                  }, false);
                                                 

                                                }

                                                else if(theelement.attachEvent){

                                                  theelement.attachEvent(&quot;onkeyup&quot;, function(e){

                                                    e = e || event;

                                                    setToolTipText(theelement);

                                                  });
                                                  
                                                }
   }
        

    function changeToolTip()

    {try{

     for(count=0; count &lt; document.forms[0].elements.length; count+=1)

       {

         theelement = document.forms[0].elements[count];

           if(theelement.options != null)

             {

                if(parseInt(theelement.options.length)!=0)

                 theelement.title=theelement.options[theelement.selectedIndex].text;

                 else

                 theelement.title=theelement.value;

             }
        }}

        catch(err){
       

        }

    }                        

//-----------------------Ends Here-----------------

var blockResponsechk=false;
var chkWin;
var chkWinMsg=&quot;&quot;;
var chkWinValue=false;
function blockClick()
            {
                 
                if(chkWin &amp;&amp; !chkWin.closed)
			{
			chkWinValue=true;
			if(chkWinMsg!=&quot;&quot;)
			{
			alert(chkWinMsg);
			}
			else {
			alert((&quot;Denomination window is not close. Please close it!&quot;).toUpperCase());
			}
			chkWin.focus();
			return false;
			}
			else {
				chkWinValue=false;
			}
               if((!document.getElementById(&quot;SaveLink&quot;)) || (!document.getElementById(&quot;SaveExitLink&quot;)))
                 {
                  if(blockResponsechk==true)
                  {
                        alert((&quot;Activity in Progress.Please wait...&quot;).toUpperCase());
                        return false;
                       
                  }
                  else
                  {                   	
                        blockResponsechk=true; 
                       setTimeout(&quot;setTimerReClick()&quot;,10000); // after 10 sec it's call
                        return true;
                  }
                }
                
               
            }
           

	function setTimerReClick()
	{		
		blockResponsechk=false;
	}

//Disable right mouse click Script
//By Maximus (maximus@nsimail.com) w/ mods by DynamicDrive
//For full source code, visit http://www.dynamicdrive.com
//added by sunny to restrict refresh using ctrl +R
$(document).ready(function () {
    $(document).on(&quot;keydown&quot;, function(e) {
        e = e || window.event;
        if (e.ctrlKey) {
            var c = e.which || e.keyCode;
            if (c == 82) {
            	alert(message);
                e.preventDefault();
                e.stopPropagation();
                return false;
            }
        }
    });
});
//add end bby sunny
 var message=&quot;FUNCTION DISABLED!&quot;;

///////////////////////////////////
function clickIE4(){
if (event.button==2){
alert(message);
return false;
}
}

function clickNS4(e){
if (document.layers||document.getElementById&amp;&amp;!document.all){
if (e.which==2||e.which==3){
alert(message);
return false;
}
}
}

if (document.layers){
document.captureEvents(Event.MOUSEDOWN);
document.onmousedown=clickNS4;

}
else if (document.all&amp;&amp;!document.getElementById){
document.onmousedown=clickIE4;
}

document.oncontextmenu=new Function(&quot;alert(message);return false&quot;);




//This function changed by Ravikant for browser compatibility
function showKeyCode(e)
{
var src=src=e.srcElement;
if(src==null || src=='null' || src==undefined )
src=e.target;
var tag = src.tagName ? src.tagName.toUpperCase() : ''; 
var typ = (tag == 'INPUT') ? src.type.toUpperCase() : ''; 
var isTextArea = (tag == 'TEXTAREA'); 
var isTextField = ((tag == 'INPUT') &amp;&amp; (typ == 'TEXT')); 
var isPassField= ((tag == 'INPUT') &amp;&amp; (typ == 'PASSWORD')); 
var isText = isTextField || isTextArea || isPassField; 

var disabled = isText ? src.disabled : false; 
var readOnly = isText ? src.readOnly : false; 

var keycode =(window.event) ? event.keyCode : e.keyCode;

if(keycode == 116)
{
alert(('Function Disabled!').toUpperCase());
event.keyCode = 0;
event.returnValue = false;
return false;
}
var browser=(navigator.userAgent).toLowerCase();
if(browser.indexOf(&quot;msie&quot;)>0){
if(!isText)
{
	if(keycode == 8)
	{
		alert(('Function Disabled!').toUpperCase());
		event.keyCode = 0;
		event.returnValue = false;
		return false;
	}
}
}

if (disabled || readOnly) 
 { 

if(keycode == 8)
{

	alert(('Function Disabled!').toUpperCase());
	event.keyCode = 0;
	event.returnValue = false;
	return false;
} 
}
}

/*Satrt Added For Bud Id:13340*/
var myWindow;
/*End Added For Bud Id:13340*/	
	function onBeforeUnloadAction()
 	{
		document.forms[0].action = &quot;userAuthAction.do?dispatchMethod=logout&quot;;
        document.forms[0].method = &quot;post&quot;;
        document.forms[0].submit();
   }
   
   window.onbeforeunload = function(e)
   {
   		if(window.event &amp;&amp; !e)
   		e=window.event;
   		
   		if((e.clientX&lt;0) ||(e.clientY&lt;0))
 		{
 			/*Satrt Added For Bud Id:13340*/
 			if (myWindow &amp;&amp; myWindow.closed) 
 			{
 		 		return onBeforeUnloadAction();
 			}
 			/*End Added For Bud Id:13340*/
 			
 			/*Satrt Comment For Bud Id:13340*/
 			
 			//return onBeforeUnloadAction();
 			
 			/*Satrt Comment For Bud Id:13340*/
 			
 		}
 	}; 
 	function debugAlert(alertMesg)
 	{
 	var debugAlert=&quot;N&quot;;
 	if(debugAlert==&quot;Y&quot;)
 	alert(alertMesg);
 	}
 	
 	function testing()
 	{
 	   // fore testing start
	
	 $.ajax({  
			type : &quot;post&quot;,   
			url : &quot;BroadcastServiceRequestServlet&quot;,   
			ontext: document.body,			
			success : function(response) {			
				  
			   }			
		}); 
	// for testing end
 	   
 	}

 
 


	
	
			
				
					  
						
				
					Hi NAVIND
					
					
				
				
				
					
				
				
					
				
				
				
				
				
					
					
						
							
							
							25-JUL-2023
							
						
					
						 
					
					
					
					
					NAVIND
					
				
					
				Logout
			
			
				
					
				
				
				
					
				
				
				
				
			
			
		
	
		
		 
		
  
	

	 

 
		$(window).on('load', function() {
			
			$(this).parents(&quot;tr&quot;).removeClass(&quot;cstmtr&quot;);
			$(document).on(&quot;click&quot;,&quot;#xyz table tr td a&quot;,function(){
			$(this).parents(&quot;tr&quot;).addClass(&quot;cstmtr&quot;);
			
			});
			$(document).on(&quot;blur&quot;,&quot;#xyz table tr td a&quot;,function(){
			$(this).parents(&quot;tr&quot;).removeClass(&quot;cstmtr&quot;);
			
			});
			$(&quot;#llmDashboardDiv table:nth-of-type(1) tr.tr_list_header td:nth-of-type(2)&quot;).css(&quot;width&quot;,&quot;40%&quot;);
			$(&quot;#llmDashboardDiv table:nth-of-type(2) td:nth-of-type(2)&quot;).css(&quot;width&quot;,&quot;40%&quot;);
			$(&quot;#llmDashboardDiv table:nth-of-type(2)  td:nth-child(3)&quot;).css(&quot;width&quot;,&quot;50%&quot;);
			//$(&quot;#llmDashboardDiv table:nth-of-type(1) tr.tr_list_header td:nth-of-type(4) b&quot;).css(&quot;padding-left&quot;,&quot;40%&quot;);
			//$(&quot;#llmDashboardDiv table:nth-of-type(1) tr&quot;).not(':first').not(&quot;.tr_list_header&quot;).find(&quot;td:nth-of-type(4)&quot;).wrapInner('&lt;span style=&quot;position:relative;right:100px; text-align: right;display:block&quot;>&lt;/span>');
			//$(&quot;#llmDashboardDiv table:nth-of-type(1) tr&quot;).not(':first').not(&quot;.tr_list_header&quot;).find(&quot;td:nth-of-type(3)&quot;).wrapInner('&lt;span style=&quot;position:relative;right:59px; text-align: right;display:block&quot;>&lt;/span>');
			//$(&quot;#llmDashboardDiv table:nth-of-type(2) tr&quot;).not(':first').not(&quot;.tr_list_header&quot;).find(&quot;td:nth-of-type(3)&quot;).wrapInner('&lt;span style=&quot;position:relative;right:42%;text-align: right;display:block;width:72%&quot;>&lt;/span>');
							var intRegex = /^\d+$/;
							var floatRegex = /^([\d\,\.]*)$/;
							var str1;
							var str = $('body td').each(function(){
							var str1 = $(this).text();
							  });
							if ($(&quot;#vetiBtn&quot;).length > 0) {
								$(&quot;.back_image_logo1&quot;).addClass(&quot;menu_add&quot;).find(&quot;img&quot;).attr(&quot;onclick&quot;, &quot;collapsePage()&quot;);
							} else {
								$(&quot;.back_image_logo1&quot;).removeClass(&quot;menu_add&quot;);
							}
							/* if ($.browser.version == 8.0) {
								var a = $.trim($(&quot;.imd_maker_space&quot;).text());
								if (a == &quot;&quot;) {
									/* console.log(&quot;ghkxgf&quot;); 
									$(&quot;.imd_maker_space,.imd_maker_user,.imd_maker_dashbord&quot;)
											.css(&quot;display&quot;, &quot;none&quot;);
									$(&quot;.imd_maker_tab&quot;).css(&quot;width&quot;, &quot;0.1%&quot;);
									$(&quot;.imd_maker_dashbord&quot;).css(&quot;width&quot;,
											&quot;1.4%&quot;);
									$(&quot;.imd_maker_user&quot;).css(&quot;width&quot;, &quot;0.8%&quot;);
									$(
											&quot;.menu-dashboard,.td_under_strip,.menu-save &quot;)
											.css(&quot;width&quot;, &quot;10%&quot;);

								}

							} */

						});
	 
	
		$(&quot;.userr&quot;).click(function() {
			$(this).parents(&quot;.username_icon&quot;).siblings(
			&quot;.username_tooltip&quot;).toggle(&quot;slow&quot;);
		});
		divHeight=&quot;&quot;;
		divHeight1=&quot;&quot;;
		$(window).on('load',function(){
			$(&quot;.text_footer&quot;).removeClass(&quot;relativefooter&quot;);
			if($(&quot;body&quot;).css(&quot;overflow-y&quot;)==&quot;scroll&quot;){$(&quot;.text_footer&quot;).addClass(&quot;relativefooter&quot;);}
			else{$(&quot;.text_footer&quot;).removeClass(&quot;relativefooter&quot;);}
    		$('body').css('margin-top', divHeight+divHeight1+'px');
			$(&quot;tr td input&quot;).each(function(){
				if($(this).attr('readonly') == 'readonly'){
				$(this).addClass(&quot;readonly_text&quot;);  }
			});
			$(&quot;input[name='addAddressButton']&quot;).click(function(){
				$(this).parents(&quot;.add_new_address_dtl&quot;).next(&quot;table&quot;).find(&quot;.readonly_text&quot;).removeClass(&quot;readonly_text&quot;);
			});
		});
	
		$(&quot;[name='prospectListHeading']&quot;).attr(&quot;id&quot;,&quot;prosListTable&quot;);
		$(&quot;#prosListTable tr td&quot;).eq(2).addClass(&quot;prosListTable_adjust2&quot;);
		$(&quot;#prosListTable tr td&quot;).eq(3).addClass(&quot;prosListTable_adjust3&quot;);
		$(window).on('load',function() {
		
			if($(&quot;.tt&quot;).text().trim()==&quot;&quot;){$(&quot;.tt&quot;).fadeOut();}
			if($(&quot;#uu&quot;).text().trim()==&quot;&quot;){$(&quot;#uu&quot;).fadeOut();}
			if($(&quot;#SDNDataId&quot;).text().trim()==&quot;&quot;){$(&quot;#SDNDataId&quot;).fadeOut();}
			$(&quot;#prosListTable tr td&quot;).eq(0).addClass(&quot;prosListTable_adjust1&quot;);
			//$(&quot;#llmDashboardDiv .static_info:nth-of-type(2) tr:nth-of-type(2) td:nth-of-type(3) b&quot;).css(&quot;padding-left&quot;,&quot;23%&quot;);
			//$(&quot;#llmDashboardDiv .static_info:nth-of-type(1) tr:nth-of-type(2) td:nth-of-type(3) b&quot;).css(&quot;padding-left&quot;,&quot;56%&quot;);
			$(&quot;#llmDashboardDiv .static_info:nth-of-type(1) tr td:nth-of-type(1)&quot;).attr(&quot;style&quot;,&quot;width:1%&quot;);
			$(&quot;#llmDashboardDiv table:nth-of-type(1) tr td:nth-of-type(1)&quot;).css(&quot;width&quot;,&quot;3%&quot;);
			$(&quot;#llmDashboardDiv .static_info:nth-of-type(1) tr td:nth-of-type(1) img&quot;).css({&quot;float&quot;:&quot;right&quot;,&quot;margin-right&quot;:&quot;4px&quot;});
			$(&quot;#llmDashboardDiv table:nth-of-type(2) tr td:nth-of-type(1)&quot;).css(&quot;width&quot;,&quot;3%&quot;);
			$(&quot;#llmDashboardDiv .static_info:nth-of-type(2) tr td:nth-of-type(1) img&quot;).css({&quot;float&quot;:&quot;right&quot;,&quot;margin-right&quot;:&quot;4px&quot;});
			$(&quot;.div_applisttable1  tr:not(:first-child)&quot;).each(function(){ 
				var linkhref = $(this).find(&quot;td:nth-child(5) a&quot;).attr('href');
				$(this).find(&quot;td:nth-child(3)&quot;).wrapInner('&lt;a href='+linkhref +'>' + '&lt;/a>');
			});
			var appname_txt = $(&quot;.applist_imd td:nth-child(3)&quot;).text();
			$(&quot;.applist_imd td:nth-child(3)&quot;).html(appname_txt);
			var divTest = $(&quot;#fset_one&quot;).height();
			var flagg1 = false;
 			$(&quot;.menu_tab tbody td a&quot;).width($(&quot;.menu_tab tbody td a&quot;).parent(&quot;td&quot;).outerWidth()-5);
			$(&quot;#verticalMenu&quot;).css({
							&quot;width&quot; : &quot;94% !important&quot;,
							&quot;position&quot; : &quot;relative&quot;,
							&quot;left&quot; : &quot;143px !important&quot;,
							&quot;overflow-y&quot; : &quot;auto&quot;,
						});

});


	 	
	
	


			








	
			
			
				
					MANUAL ADVICE MAKER
				
			
			
			
    
	
			
		
		 
			My Dashboard
		 
	
		
	        		
		
			
					
				 

			
            
			    
                
				
				
				
				
				
				
				  
				
				
				
				
				
				
				
				  
				   
				   
					
					My Worklist
					
					
					   
				
				 
				 
				 
				
				
				
				
				
				
				
				
				
				
				
				
				
				
			
				
				
				

				
			  
	
				
		    
		    





						
						
						
						
						
						
						
						
						
						
						
						
						
						
						  
						
						
						
						 
						
						
						
						
						
						 Save
						 
						
						
						
						
						
						
						
						
						
						
						
						
						
						
						
						
						
						
						
						
						
						

						
										
								    
								    
						


						
						
						
						
							
						 
						
						 
						 
						
						
						 Save &amp; Exit
							 
							
							
							
							
						
						
						
						

						
						
						
						
						
						


				
			
			
		
		
								
		
	


	      











var inactiveInterval = 1800
var loanAmt = &quot;&quot;;

	//This function is changed by Ravikant for browser compatibility.
	function callWfp()
	{
		var sessionStatus = checkinterval(inactiveInterval);		
		if(sessionStatus==&quot;Y&quot;)
		{	 
			var browser=(navigator.userAgent).toLowerCase();
			//alert(browser);
			if(browser.indexOf(&quot;chrome&quot;)>0)
			{	
				var targetWin = window.open(&quot;workFlowProc.do?actionPerformed=displayWorkFlowPage&amp;rurl=&quot;+Math.random(),&quot;workFlow&quot;,&quot;dialogWidth:1000px;dialogHeight:500px&quot;);
				targetWin.focus();
			}else{
				window.showModalDialog(&quot;workFlowProc.do?actionPerformed=displayWorkFlowPage&amp;rurl=&quot;+Math.random(),&quot;workFlow&quot;,&quot;dialogWidth:1000px;dialogHeight:500px&quot;);
			}
		}
		else
		{
			
			window.location=&quot;userAuthAction.do?dispatchMethod=logout&quot;;
		}
	}
	
	function showNotePad()
	{
		var sessionStatus = checkinterval(inactiveInterval);		
		if(sessionStatus==&quot;Y&quot;)
		{	 
			var browser=(navigator.userAgent).toLowerCase();
			//alert(browser);
			if(browser.indexOf(&quot;chrome&quot;)>0)
			{	
				var targetWin = window.open(&quot;notepadAction.do?actionPerformed=displayNotePadInfo&amp;staticCall=Y&amp;rurl=&quot;+Math.random(),&quot;workFlow&quot;,&quot;dialogWidth:1030px;dialogHeight:600px&quot;);
				targetWin.focus();
			}else{
				window.showModalDialog(&quot;notepadAction.do?actionPerformed=displayNotePadInfo&amp;staticCall=Y&amp;rurl=&quot;+Math.random(),&quot;workFlow&quot;,&quot;dialogWidth:1030px;dialogHeight:600px&quot;);
			}
		}
		else
		{
			
			window.location=&quot;userAuthAction.do?dispatchMethod=logout&quot;;
		}
	}
	
    	/* function for the  Portable Search Div Hide And Show Button(+/-)  date 14 /06/2012  By Ambar Gupta */
	function showdiv()
	{	
	 
           var Lgd = document.getElementById(&quot;searchLgd&quot;);
        //mayank Agrawal for compatiblity
            if (document.getElementById(&quot;one&quot;).style.display == &quot;block&quot; || document.getElementById(&quot;one&quot;).style.display==&quot;&quot;)
            {   
                
                 
                  
                  			   
				    document.getElementById(&quot;imageUpload&quot;).style.display = &quot;block&quot;;
				  
				  
				 
				
				document.getElementById(&quot;one&quot;).style.display = &quot;none&quot;; 
				var temp=document.getElementsByName(&quot;btn_one&quot;);
				
				
				document.getElementById(&quot;fset_one&quot;).style.border= &quot;0px solid #83b0ec&quot;;
				Lgd.innerHTML = '&lt;input type=&quot;button&quot; class=&quot;imageBottonDown&quot;  name=&quot;btn_one&quot; id=&quot;btn_one&quot; onclick=&quot;javascript:showdiv()&quot;  title=&quot;Maximize the Common Information&quot; style=&quot;cursor: pointer;border: none&quot;>&lt;/input>&amp;nbsp;Static Info';
			}
            else if (document.getElementById(&quot;one&quot;).style.display == &quot;none&quot;)
			{ 
			    
				document.getElementById(&quot;one&quot;).style.display = &quot;block&quot;;
			    
                 
                   
                  			   
				    document.getElementById(&quot;imageUpload&quot;).style.display = &quot;block&quot;;
				  
				  
				 
				
				var temp=document.getElementsByName(&quot;btn_one&quot;);
				
				document.getElementById(&quot;fset_one&quot;).style.border= &quot;1px solid #83b0ec&quot;;
				Lgd.innerHTML = '&lt;input type=&quot;button&quot; class=&quot;imageBottonUp&quot;  name=&quot;btn_one&quot; id=&quot;btn_one&quot;  onclick=&quot;javascript:showdiv()&quot;  title=&quot;Minimize the Common Information&quot; style=&quot;cursor: pointer;border: none&quot;>&lt;/input>&amp;nbsp;Static Info';
			}	
						
	}
	      /*   Function End Here */	
// 1.0.0.5 start 	      
function rentalPayScheduleReport()
  	{
	window.open(&quot;rentalPayScheduleReport.sprg&quot;);	
  	}	      	
// 1.0.0.5 end 	

//1.0.0.12 start
 function getReschHistScreen()
{
	 
	var paramList='1000008259';
    var url =   &quot;dynamicWindow.do?actionPerformed=dynamicWindow&amp;windowId=1000000005&amp;paramList=&quot;+paramList+&quot;&quot;;
	   
	   var self=window.open(url,&quot;dynamicWindow&quot;,&quot;width=800px,height=500px,left=400px,titlebar=yes,scrollbars=yes,toolbar=no,maximize=no,menubar=no,minimize=no,statusbar=no&quot;);
	   
} 
//1.0.0.12 end








 

 


  	   
	

		
				PROSPECT NO	
			DMFIN1000008259
		
			
			
			CUSTCODE		
			CNCIMF000003205
		
		
		
			CUST NAME
			ROHAN  TESTQA
		
		
		
		
		
		
		
		
		CUST LIMIT CODE
			LMCST0000006150
			
			
			
			
		QUOTATION CODE
			QU00022832
			
			
			
			
			
		
			
			GROUP NAME	
			
			PROJECT TEST RO
	    
			
		
			CAPITALISED FLAG
			Y
			
			
			
			
			PRODUCT
			Finance Lease
			
		
			
			SCHEME
			ONLY FIXED FINA...ONLY FIXED FINANCE LEASE
		
		
			
			ASSET CATEGORY
			VEHICLE
		
			
			AGREEMENT TYPE
			BI PARTY
		
		
		
		
		
			SANCTIONED AMT	
			115.00
		
		
		
			
             LEASE SANCTIONE...LEASE SANCTIONED DATE
			13-JAN-2023			
			
		
		
		
		
		
		
			
			
			LEASE RENTAL AMT	
				
			12.00
		
		
			
			RENTAL TYPE
			EQUATED
		
		
		
  
	  
			
			ENTITY TYPE
			INDIVIDUAL
		
			
		
		
		
			
				NO. OF INSTALMENTS
				10
			
		
		
			BRANCH
			HEAD OFFICE
		
	
		
			APPLICATION FOR...APPLICATION FORM NO.
			APPFORM0008259
		
		
		
		
			
			CASHFLOW IRR
			
			5
		
		
		
		
			
			RENTAL START DATE
			
			07-MAR-2023
		
		
		
		
	
	
	
		
		
		
		
			DM STATUS
		
		
		
		
		DISBURSED
		
		
		
		
		
			LEASE APPLICATI...LEASE APPLICATION DATE
		
		
			13-JAN-2023
		
		
		
		
		
		
			
			RENTAL FREQUENCY
			
			MONTHLY
		
		
		
			FORECLOSURE METHOD
			BOOK VALUE
			
			
		
			NPA STAGE
			REGULAR
		
		
		 
	   
    
	
		DAYS SINCE CUST...DAYS SINCE CUSTOMER SIGNED
		
	
	
		
		DAYS IN CURRENT...DAYS IN CURRENT STAGE
		
	
	
	
      
  	
	     
    
				
		
		
	
	    
   
	  
		
	
		
		
		
			TRANSFERED FROM DM
			
			
			
			
			
		
			RV TYPE
			100+1
			
			
			
		
			RV AMOUNT
			                   5.00
			
			
			
			
				
			
			
					    
		    
			
			RESCHEDULED
			
			N
			
			
			
			
			
			
		
			NOTEPAD
			
		
			
		
	
	
	
		
		
		
		   
	
  
  
  
  
  
  
  
 
  
		
			
		
		  
		
 
		   
		   		


 
  
     
     
    
	 
	
	
	
	
	
    



		


	 		 
	  	 




 



	$(document).ready(function(){

		var linkhref = $(&quot;.noti_link span a&quot;).attr('href');
				$(&quot;.prose2 span&quot;).wrapInner('&lt;a href='+linkhref +'>' + '&lt;/a>');
				
	
				var heightDiv = &quot;80px&quot;;
				
			
				$(&quot;#one&quot;).addClass(&quot;onecustom&quot;);
				$(&quot;#one&quot;).toggleClass(&quot;tgling&quot;).css(&quot;height&quot;,&quot;30px&quot;);
				$(document).on(&quot;click&quot;,&quot;.tgl_nbtn&quot;,function(){
					$(&quot;#one&quot;).toggleClass(&quot;tgling&quot;);
		//1.0.0.4 start			
				if(document.getElementById(&quot;customerLimitStaticInfo&quot;))
				 {
				   heightDiv= Number($(&quot;#customerLimitStaticInfo&quot;).height())+20;
				 }
				 else  if(document.getElementById(&quot;groupStaticInfo&quot;))
				 {
				   heightDiv= Number($(&quot;#groupStaticInfo&quot;).height())+20;
				 }
				  else  if(document.getElementById(&quot;customerStaticInfo&quot;))
				 {
				   heightDiv= Number($(&quot;#customerStaticInfo&quot;).height())+20;
				 }
				 else  if(document.getElementById(&quot;cvLimitStaticInfo&quot;))
				 {
				   heightDiv= Number($(&quot;#cvLimitStaticInfo&quot;).height())+20;
				 }
				 else  if(document.getElementById(&quot;losStaticInfo&quot;))
				 {
				   heightDiv= Number($(&quot;#losStaticInfo&quot;).height())+40;
				 }
			//1.0.0.4 end			
					$(&quot;#one&quot;).css(&quot;height&quot;,heightDiv);
					$(&quot;#one.tgling&quot;).css(&quot;height&quot;,&quot;30px&quot;);
					$(this).toggleClass(&quot;tglingSpan&quot;);
				});

			$(&quot;#fset_one .static_info_head&quot;).each(function(){
			if($(this).text().trim().length>=20) 
			{ 
			var originText1=$(this).text();   
			var cropText1=originText1.substring(0,15); 
			$(this).text(cropText1+&quot;...&quot;); 
			$(this).after(&quot;&lt;span class='hoverStaticInfo' style='display:none'>&quot;+originText1+&quot;&lt;/span>&quot;);$(this).addClass(&quot;positioned_div&quot;);
			}
		

});



	
	});

	
		
			


	
		
		
	var contextPath = &quot;/lease&quot;;

		
		
		
		
		
/* All Changes have done by Ambar Gupta for Persist the Stae of menu link and also have Show hide functionalty */
var isCollapsed = false;
//Changed by Ravikant for browser compatibility on 29-Dec-14.
function collapsePage()
{
	var menuDiv = document.getElementById(&quot;verticalMenu&quot;);
	var barDiv = document.getElementById(&quot;linkTD&quot;);
	var vetiTd = document.getElementById(&quot;vetiTd&quot;);
	if(isCollapsed)
	{
		menuDiv.style.display = &quot;block&quot;;
// 		document.getElementById('dasiTd').style.width = &quot;80%&quot;;
// 		document.getElementById('vetiTd').style.width = &quot;20%&quot;;
		setDimension(&quot;1&quot;);
		
		barDiv.innerHTML = '&amp;nbsp;&amp;nbsp;&amp;nbsp;&amp;nbsp;VERTICAL MENU &amp;nbsp;&amp;nbsp;&amp;nbsp;&amp;nbsp;&amp;nbsp;&amp;nbsp;&amp;nbsp;&amp;nbsp;&lt;input type=&quot;button&quot; class=&quot;imageBottonLeft&quot;  onclick=&quot;collapsePage()&quot; title=&quot;Minimize Vertical Menu&quot; style=&quot;cursor: pointer;border:none;margin-bottom:0px;margin-top:0px;align:right;&quot; id=&quot;vetiBtn&quot;>';
		isCollapsed = false;
		vetiTd.style.backgroundImage = &quot;&quot;;
	}
	else
	{	
		barDiv.innerHTML = '&lt;input type=&quot;button&quot; class=&quot;imageBottonRight&quot;  onclick=&quot;collapsePage()&quot; title=&quot;Maximize Vertical Menu&quot; style=&quot;cursor: pointer;border: none;margin-bottom:0px;margin-top:0px;&quot; id=&quot;vetiBtn&quot; align=&quot;right&quot;>';
		menuDiv.style.display = &quot;none&quot;;
// 		document.getElementById('dasiTd').style.width = &quot;99%&quot;;
// 		document.getElementById('vetiTd').style.width = &quot;1%&quot;;
		setDimension(&quot;2&quot;);
		newImage = &quot;url(images/VERTICALMENUBAR.gif)&quot;;
		vetiTd.style.backgroundImage = newImage;
		isCollapsed = true;
	}
	menuDiv.style.width=&quot;230&quot;;
	//menuDiv.style.height=&quot;100%&quot;;
	menuDiv.style.overflow=&quot;auto&quot;;
}
function changecolor1()
	{
	   var barBtn = document.getElementById(&quot;vetiBtn&quot;);
	   barBtn.style.backgroundColor='#00FF00';
	}
	function changecolor()
	{ 
	   
	   var barBtn = document.getElementById(&quot;vetiBtn&quot;);
	   barBtn.style.backgroundColor='';
	}
	//Added by Ravikant for browser compatibility on 29-Dec-14.
	function setDimension(d)
	{
		var x = screen.width;
		var y = screen.height;
		if(d == &quot;1&quot;)
		{
			//document.getElementById(&quot;vetiTd&quot;).style.width = (x*0.2) + &quot;px&quot;; 
		   // document.getElementById(&quot;dasiTd&quot;).style.width = (x-(x*0.2)) + &quot;px&quot;;	
		    
		   // document.getElementById(&quot;vetiTd&quot;).style.height = (y-(y*0.01)) + &quot;px&quot;;  
		   // document.getElementById(&quot;dasiTd&quot;).style.height = (y-(y*0.01)) + &quot;px&quot;;   
	    }
	    else if(d == &quot;2&quot;)
	    {
	    	//document.getElementById(&quot;vetiTd&quot;).style.width = (x*0.01) + &quot;px&quot;; 
		   // document.getElementById(&quot;dasiTd&quot;).style.width = (x-(x*0.01)) + &quot;px&quot;;
		    
		   // document.getElementById(&quot;vetiTd&quot;).style.height = (y-(y*0.01)) + &quot;px&quot;;  
		  //  document.getElementById(&quot;dasiTd&quot;).style.height = (y-(y*0.01)) + &quot;px&quot;;  
	    }
	    //alert(document.getElementById(&quot;vetiTd&quot;).style.width+&quot; &quot;+document.getElementById(&quot;dasiTd&quot;).style.width+&quot; &quot;+document.getElementById(&quot;dasiTd&quot;).style.height);
	}

	



		
			

			
				
				
			
			
				
				UTILITYCHANGE PASSWORDCUSTOMERNEWCUSTOMERPENDING ACTIVITIESAPPLICANTDEDUPENOTEPADDOCUMENTCR REVIEWCUSTOMER EDITCUSTOMER VIEWERGROUPNEW GROUPGROUP EDITLIMIT(LEASE)CUSTOMER LIMITNEWCUSTOMER LIMITPENDING ACTIVITIESEDITCR DECISIONCUSTOMER LIMIT VIEWERGROUP LIMITGROUP LIMIT VIEWERQUOTATIONNEWLEASE PROPOSALPENDING ACTIVITIESASSET DETAILSQUOTATIONINSURANCECASHFLOWREGISTRATION DETAILSACCEPTANCEACQUISITIONVIEWERDM QUOTATIONNEWDM APPLICATIONPENDING ACTIVITIESASSET DETAILSQUOTATIONCASHFLOWDM OFFLINE ACTIONDM SANCTIONCAPITALISATION  MAKERDM CANCELLATIONMAKERVIEWERPAYMENTMAKERVIEWERDM VIEWERLMSINVOICEINVOICEDM PDEMAKERRECEIPTMAKERPAYMENT HISTORYRECEIPT CANCELLATIONMAKERVIEWERREFUNDMAKERVIEWERMANUAL ADVICEMAKERKNOCK OFFMAKERVIEWERINSTR MANAGEMENTPDC/ECS GENERATEMAKERPDC/ECS EDITMAKERVIEWERWAIVE OFFMAKERVIEWERCLOSE / FORECLOSEMAKERVIEWERBATCH PROCESSGENERATIONRESCHEDULING CASEMAKERREPORTSCREDIT OPERATION REPORTANNEXES OLAUDIT CONFIRMATIONBOARD OF RESOLUTIONBOARD RESOLUTION CORPORATE GUARANTORCANCELLATION SALARY DEDUCTION FORMCANCELLATION STANDING ORDER FORMCERTIFICAT DE GAGE SECOND HANDCHANGE IN ENGINE NUMBERCHANGE IN REGISTRATION NUMBERCHANGE OF NAME OF HORSEPOWERCORPORATE GUARANTEECOVER LETTERDIRECT DEBIT FORMHORSEPOWER FULLLEASE AGREEMENTLEASE AGREEMENT OLLETTER OF UNDERTAKINGLOST HORSEPOWERNO LIABILITY LETTEROFFER LETTERREPAYMENT SCHEDULERIGHT OF SET OFF LETTERRV LETTERRV SALES DEED AND CERTIFICATE OF GAGESALARY DEDUCTION FORMSALES DEEDSETTLEMENT LETTERSTANDING ORDER FORMSUBORDINATION OF SHAREHOLDERS LOANSUPPLIER LETTERACCOUNTING REPORTACCRUAL REPORTASSET RECEIPT REPORTAUTO RESCHEDULE STATUS REPORTBASE_RATE_CHANGE_TRACK_REPORTCLIENT MIS REPORTCUSTOMER DETAIL REPORTDOCUMENT PENDING AND EXPIRY REPORTINSURANCE REPORTINVOICE REPORTLEAD FOLLOW UPLEAD STATUSLEASE REGISTRATION TRACKINGLIMIT EXPIRYLPI REPORTMATURING AGREEMENTS REPORTOUTSTANDING DUE REPORTRV DUE REPORTSCHEDULED REPORTSTRANSACTION REPORTWAIVER_REPORTMANUAL ADVICELEASE PROPOSALPO REPORTQUOTATION DETAIL REPORTRESCHEDULING REPORTBATCH DTL REPORTSTATEMENT OF ACCOUNTBATCH PRESENTATION REPORTTRANSACTION REPORTTRIAL BALANCEVENDOR PAYMENT REPORTBATCH STATUS REPORTBATCH UPLOAD ISSUES REPORTBULK INVOICE REPORTCOVERNOTE REPORTAGEING REPORTBRANCH WISE SUMMARYUSER WISE SUMMARYCUSTOMER LIMIT REPORTFORECLOSURE REPORTALCO REPORTGROUP CUSTOMER REPORTGROUP LIMIT REPORTLEASE CAM REPORTLEASE CREDIT NOTEVAT INVOICEMASTERSMAKERSALESMANAGERAUTHORBATCH UPLOADMAKERNEGATIVE LISTSDNTALIBANAL-QAIDAMCIBCAUTION LISTAP INT GL MAP CONFIGBULK RECEIPTINSTRUMENT MANAGEMENTLEASE REGISTRATION TRACKINGOFFLINE INSURANCE DETAILSPDC/ECSQT FLEET DETAIL UPLOADLEASE ASSET PRICE MST UPLOADLEASE ASSET VENDOR UPLOADLEASE VENDOR GSTIN UPLOADMANUAL VOUCHER UPLOADCUSTOMER INVOICE CONFIG UPLOADLEASE BATTERY PRICE UPLOADLEASE TYRE PRICE UPLOADMANUAL ADVICE UPLOADLEASE ASSET CESSMAPPING UPLOADLEASE ASSET VARIANT UPLOADLEASE FC AMNT FORMULA UPLOADLEASE SERVICE CHRG AMT UPLOADLEASE VENDOR BANK UPLOADLEASE VENDOR MAPPING UPLOADLEASE MAINT SLABS UPLOADBULK CLOSURECHARGE X GST UPLOADLEASE RESIDUAL VALUE UPLOADAUTHORNEGATIVE LISTCAUTION LISTAP INT GL MAP CONFIGBULK CLOSUREBULK RECEIPTCUSTOMER RECEIPTLEASE ASSET PRICE MST UPLOADPDC/ECSLEASE VENDOR BANK UPLOADMANUAL ADVICE UPLOADLEASE ASSET VARIANT UPLOADLEASE FC AMNT FORMULA UPLOADLEASE SERVICE CHRG AMT UPLOADLEASE ASSET VENDOR UPLOADLEASE VENDOR GSTIN UPLOADMANUAL VOUCHER UPLOAD AUTHORLEASE BATTERY PRICE UPLOADLEASE TYRE PRICE UPLOADLEASE MAINT SLABS UPLOADCHARGE X GST UPLOADLEASE RESIDUAL VALUE UPLOADVIEWERNEGATIVE LISTSDNTALIBANAL-QAIDAMCIBCAUTION LISTAP INT GL MAP CONFIGLEASE REGISTRATION TRACKINGOFFLINE INSURANCE DETAILSLEASE ASSET PRICE MST UPLOADMANUAL VOUCHER UPLOADCUSTOMER INVOICE CONFIG UPLOADMANUAL ADVICE UPLOADLEASE ASSET VARIANT UPLOADLEASE FC AMNT FORMULA UPLOADLEASE SERVICE CHRG AMT UPLOADLEASE VENDOR BANK UPLOADLEASE VENDOR MAPPING UPLOADBULK RECEIPTCUSTOMER RECEIPT VIEWERINSTRUMENT MANAGEMENT VIEWERRESCH VIEWERBULK CLOSURE VIEWERPDC/ECSLEASE ASSET VENDOR UPLOADLEASE VENDOR GSTIN UPLOADLEASE BATTERY PRICE UPLOADLEASE TYRE PRICE UPLOADLEASE MAINT SLABS UPLOADCHARGE X GST UPLOADLEASE RESIDUAL VALUE UPLOAD
					
					var menuDataXml = &quot;&lt;ROWSET> &lt;ROW>  &lt;ACTION_ID>1000000001&lt;/ACTION_ID>  &lt;PARENT_ID/>  &lt;DISPLAY_NAME>UTILITY&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>UTILITY&lt;/MENU_TYPE>  &lt;SEQUENCE>10&lt;/SEQUENCE>  &lt;SCREEN_NAME>UTILITY&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>10&lt;/SEQ_NO>  &lt;PATH>-110&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000002&lt;/ACTION_ID>  &lt;PARENT_ID>1000000001&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CHANGE PASSWORD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>forwardFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000002&amp;amp;screenName=CHANGE PASSWORD&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>CHANGE PASSWORD&lt;/MENU_TYPE>  &lt;SEQUENCE>2&lt;/SEQUENCE>  &lt;SCREEN_NAME>AAA&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>changePassword.do?actionPerformed=displayChangePassword&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>2&lt;/SEQ_NO>  &lt;PATH>-110-22&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000002041&lt;/ACTION_ID>  &lt;PARENT_ID/>  &lt;DISPLAY_NAME>CUSTOMER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>CUSTOMER&lt;/MENU_TYPE>  &lt;SEQUENCE>11&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>11&lt;/SEQ_NO>  &lt;PATH>-111&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000010289&lt;/ACTION_ID>  &lt;PARENT_ID>1000002041&lt;/PARENT_ID>  &lt;DISPLAY_NAME>NEW  &lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>NEW&lt;/MENU_TYPE>  &lt;SEQUENCE>22&lt;/SEQUENCE>  &lt;SCREEN_NAME>NEW&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>22&lt;/SEQ_NO>  &lt;PATH>-111-222&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000002042&lt;/ACTION_ID>  &lt;PARENT_ID>1000010289&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>NewCustomerDetails.do?actionPerformed=displayCustomer&amp;amp;screenId=1000002042&amp;amp;screenName=CUSTOMER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>CUSTOMER&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>NewCustomerDetails.do?actionPerformed=displayCustomer&amp;amp;screenId=1000002042&amp;amp;screenName=CUSTOMER&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>CustomerDetails&lt;/REQUEST_PAGE>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-111-222-33&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000002049&lt;/ACTION_ID>  &lt;PARENT_ID>1000002041&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER EDIT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>applicantListFrmNavAction.do?actionPerformed=displayApplicantListInfo&amp;amp;screenId=1000002049&amp;amp;screenName=CUSTOMER EDIT&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>CUSTOMER&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER EDIT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>applicantList.do?actionPerformed=displayCustomerList&amp;amp;screenId=1000000079&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-111-23&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000002043&lt;/ACTION_ID>  &lt;PARENT_ID>1000002041&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>applicantListFrmNavAction.do?actionPerformed=displayApplicantListInfo&amp;amp;screenId=1000002043&amp;amp;screenName=CUSTOMER VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>CUSTOMER&lt;/MENU_TYPE>  &lt;SEQUENCE>4&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>applicantList.do?actionPerformed=displayCustomerList&amp;amp;screenId=1000002043&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>4&lt;/SEQ_NO>  &lt;PATH>-111-24&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000010288&lt;/ACTION_ID>  &lt;PARENT_ID>1000002041&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PENDING ACTIVITIES &lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PENDING ACTIVITIES&lt;/MENU_TYPE>  &lt;SEQUENCE>5&lt;/SEQUENCE>  &lt;SCREEN_NAME>PENDING ACTIVITIES&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>5&lt;/SEQ_NO>  &lt;PATH>-111-25&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000010283&lt;/ACTION_ID>  &lt;PARENT_ID>1000010288&lt;/PARENT_ID>  &lt;DISPLAY_NAME>APPLICANT &lt;/DISPLAY_NAME>  &lt;ACTION_LINK>applicantListFrmNavAction.do?actionPerformed=displayApplicantListInfo&amp;amp;screenId=1000010283&amp;amp;screenName=CUSTOMER DETAIL&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>APPLICANT DETAIL&lt;/MENU_TYPE>  &lt;SEQUENCE>11&lt;/SEQUENCE>  &lt;SCREEN_NAME>APPLICANT DETAIL&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>applicantList.do?actionPerformed=displayCustomerList&amp;amp;screenId=1000000079&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>11&lt;/SEQ_NO>  &lt;PATH>-111-25-311&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000010284&lt;/ACTION_ID>  &lt;PARENT_ID>1000010288&lt;/PARENT_ID>  &lt;DISPLAY_NAME>DEDUPE &lt;/DISPLAY_NAME>  &lt;ACTION_LINK>applicantListFrmNavAction.do?actionPerformed=displayApplicantListInfo&amp;amp;screenId=1000010284&amp;amp;screenName=CUSTOMER DETAIL&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>DEDUPE&lt;/MENU_TYPE>  &lt;SEQUENCE>12&lt;/SEQUENCE>  &lt;SCREEN_NAME>DEDUPE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>dedupe.do?actionPerformed=displayDedupeCustomerList&amp;amp;screenId=1000000080&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>12&lt;/SEQ_NO>  &lt;PATH>-111-25-312&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000010285&lt;/ACTION_ID>  &lt;PARENT_ID>1000010288&lt;/PARENT_ID>  &lt;DISPLAY_NAME>NOTEPAD &lt;/DISPLAY_NAME>  &lt;ACTION_LINK>applicantListFrmNavAction.do?actionPerformed=displayApplicantListInfo&amp;amp;screenId=1000010285&amp;amp;screenName=CUSTOMER DETAIL&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>NOTEPAD&lt;/MENU_TYPE>  &lt;SEQUENCE>13&lt;/SEQUENCE>  &lt;SCREEN_NAME>NOTEPAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>notepadAction.do?actionPerformed=displayNotePadInfo&amp;amp;screenId=1000000084&amp;amp;mode=E&amp;amp;disName=NOTEPAD&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>13&lt;/SEQ_NO>  &lt;PATH>-111-25-313&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000010287&lt;/ACTION_ID>  &lt;PARENT_ID>1000010288&lt;/PARENT_ID>  &lt;DISPLAY_NAME>DOCUMENT &lt;/DISPLAY_NAME>  &lt;ACTION_LINK>applicantListFrmNavAction.do?actionPerformed=displayApplicantListInfo&amp;amp;screenId=1000010287&amp;amp;screenName=CUSTOMER DETAIL&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>DOCUMENT&lt;/MENU_TYPE>  &lt;SEQUENCE>15&lt;/SEQUENCE>  &lt;SCREEN_NAME>DOCUMENTS&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>documentEntryAction.do?actionPerformed=displayDocumentEntry&amp;amp;screenId=1000000082&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>15&lt;/SEQ_NO>  &lt;PATH>-111-25-315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100000039&lt;/ACTION_ID>  &lt;PARENT_ID>1000010288&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CR REVIEW&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>applicantListFrmNavAction.do?actionPerformed=displayApplicantListInfo&amp;amp;screenId=1100000039&amp;amp;screenName=CR DECISION&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>CR REVIEW&lt;/MENU_TYPE>  &lt;SEQUENCE>5&lt;/SEQUENCE>  &lt;SCREEN_NAME>CREDIT REVIEW&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>persDescAction.do?actionPerformed=displayPersonalDiscussionPage&amp;amp;screenId=1000002044&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>5&lt;/SEQ_NO>  &lt;PATH>-111-25-35&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106515&lt;/ACTION_ID>  &lt;PARENT_ID/>  &lt;DISPLAY_NAME>GROUP&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>GROUP&lt;/MENU_TYPE>  &lt;SEQUENCE>111&lt;/SEQUENCE>  &lt;SCREEN_NAME>GROUP&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>newInsurance&lt;/REQUEST_PAGE>  &lt;SEQ_NO>111&lt;/SEQ_NO>  &lt;PATH>-1111&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106516&lt;/ACTION_ID>  &lt;PARENT_ID>1100106515&lt;/PARENT_ID>  &lt;DISPLAY_NAME>NEW GROUP&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>groupAction.do?actionPerformed=displayGroup&amp;amp;screenId=1100106516&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>GROUP&lt;/MENU_TYPE>  &lt;SEQUENCE>23&lt;/SEQUENCE>  &lt;SCREEN_NAME>NEW GROUP&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>23&lt;/SEQ_NO>  &lt;PATH>-1111-223&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106517&lt;/ACTION_ID>  &lt;PARENT_ID>1100106515&lt;/PARENT_ID>  &lt;DISPLAY_NAME>GROUP EDIT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>groupAction.do?actionPerformed=displayGroup&amp;amp;screenId=1100106517&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>GROUP&lt;/MENU_TYPE>  &lt;SEQUENCE>24&lt;/SEQUENCE>  &lt;SCREEN_NAME>GROUP EDIT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>24&lt;/SEQ_NO>  &lt;PATH>-1111-224&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200006000&lt;/ACTION_ID>  &lt;PARENT_ID/>  &lt;DISPLAY_NAME>LIMIT(LEASE)&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMITS&lt;/MENU_TYPE>  &lt;SEQUENCE>112&lt;/SEQUENCE>  &lt;SCREEN_NAME>LIMIT(LEASE)&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>112&lt;/SEQ_NO>  &lt;PATH>-1112&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200007001&lt;/ACTION_ID>  &lt;PARENT_ID>1200006000&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER LIMIT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMITS&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER LIMIT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-1112-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200006001&lt;/ACTION_ID>  &lt;PARENT_ID>1200006000&lt;/PARENT_ID>  &lt;DISPLAY_NAME>GROUP LIMIT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMITS&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>GROUP LIMIT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-1112-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200007002&lt;/ACTION_ID>  &lt;PARENT_ID>1200007001&lt;/PARENT_ID>  &lt;DISPLAY_NAME>NEW&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMITS&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>NEW&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>customerLimit.do?actionPerformed=displayNewCustomerLimit&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-1112-21-31&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200007037&lt;/ACTION_ID>  &lt;PARENT_ID>1200007002&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER LIMIT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>customerLimit.do?actionPerformed=displayNewCustomerLimit&amp;amp;screenId=1200007037&amp;amp;screenName=CUSTOMER LIMIT&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMITS&lt;/MENU_TYPE>  &lt;SEQUENCE>11&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER LIMIT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>customerLimit.do?actionPerformed=displayNewCustomerLimit&amp;amp;screenId=1200007037&amp;amp;screenName=CUSTOMER LIMIT&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>customerLimit.do?actionPerformed=displayNewCustomerLimit&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>11&lt;/SEQ_NO>  &lt;PATH>-1112-21-31-411&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200007003&lt;/ACTION_ID>  &lt;PARENT_ID>1200007001&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PENDING ACTIVITIES&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMITS&lt;/MENU_TYPE>  &lt;SEQUENCE>2&lt;/SEQUENCE>  &lt;SCREEN_NAME>PENDING ACTIVITIES&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>2&lt;/SEQ_NO>  &lt;PATH>-1112-21-32&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200007004&lt;/ACTION_ID>  &lt;PARENT_ID>1200007003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>EDIT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>customerLimit.do?actionPerformed=customerLimitPendingSearchList&amp;amp;screenId=1200007004&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMITS&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>EDIT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>customerLimit.do?actionPerformed=displayCustomerLimitEdit&amp;amp;screenId=1200007030&amp;amp;screenName=CUSTOMER LIMIT EDIT&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-1112-21-32-41&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200007005&lt;/ACTION_ID>  &lt;PARENT_ID>1200007003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CR DECISION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>customerLimit.do?actionPerformed=customerLimitPendingSearchList&amp;amp;screenId=1200007005&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMITS&lt;/MENU_TYPE>  &lt;SEQUENCE>2&lt;/SEQUENCE>  &lt;SCREEN_NAME>CR DECISION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>creditDecision.do?actionPerformed=displayCreditDecisionPage&amp;amp;screenId=1200007034&amp;amp;screenName=CR DECISION&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>2&lt;/SEQ_NO>  &lt;PATH>-1112-21-32-42&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200006006&lt;/ACTION_ID>  &lt;PARENT_ID>1200006001&lt;/PARENT_ID>  &lt;DISPLAY_NAME>GROUP LIMIT VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>groupLimitAction.do?actionPerformed=groupLimitPendingSearch&amp;amp;screenId=1200006006&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMITS&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>GROUP LIMIT VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>groupLimitAction.do?actionPerformed=displayGroupLimitEdit&amp;amp;screenId=1200006030&amp;amp;screenName=EDIT&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-1112-21-33&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200007006&lt;/ACTION_ID>  &lt;PARENT_ID>1200007001&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER LIMIT VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>customerLimit.do?actionPerformed=customerLimitPendingSearchList&amp;amp;screenId=1200007006&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMITS&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER LIMIT VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>customerLimit.do?actionPerformed=displayCustomerLimitEdit&amp;amp;screenId=1200007030&amp;amp;screenName=EDIT&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-1112-21-33&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003000&lt;/ACTION_ID>  &lt;PARENT_ID/>  &lt;DISPLAY_NAME>QUOTATION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>12&lt;/SEQUENCE>  &lt;SCREEN_NAME>QUOTATION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>12&lt;/SEQ_NO>  &lt;PATH>-112&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003001&lt;/ACTION_ID>  &lt;PARENT_ID>1200003000&lt;/PARENT_ID>  &lt;DISPLAY_NAME>NEW&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>NEW&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospetFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>newProspect&lt;/REQUEST_PAGE>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-112-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003002&lt;/ACTION_ID>  &lt;PARENT_ID>1200003001&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE PROPOSAL&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>newEnquiry.sprg?screenId=1200003002&amp;amp;screenName=NEW ENQUIRY&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>91&lt;/SEQUENCE>  &lt;SCREEN_NAME>NEW ENQUIRY&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>newEnquiry.sprg&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>newProspect&lt;/REQUEST_PAGE>  &lt;SEQ_NO>91&lt;/SEQ_NO>  &lt;PATH>-112-21-391&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003003&lt;/ACTION_ID>  &lt;PARENT_ID>1200003000&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PENDING ACTIVITIES&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>23&lt;/SEQUENCE>  &lt;SCREEN_NAME>PENDING ACTIVITES&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>23&lt;/SEQ_NO>  &lt;PATH>-112-223&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003005&lt;/ACTION_ID>  &lt;PARENT_ID>1200003003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>ASSET DETAILS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>quotationList.sprg?screenId=1200003005&amp;amp;screenName=ASSET&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>ASSET&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>assetDetails.sprg?screenId=1200003012&amp;amp;screenName=ASSET&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-112-223-31&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003006&lt;/ACTION_ID>  &lt;PARENT_ID>1200003003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>QUOTATION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>quotationList.sprg?screenId=1200003006&amp;amp;screenName=QUOTATION&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>11&lt;/SEQUENCE>  &lt;SCREEN_NAME>QUOTATION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>quotationDetails.sprg?screenId=1200003013&amp;amp;screenName=QUOTATION&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>11&lt;/SEQ_NO>  &lt;PATH>-112-223-311&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003007&lt;/ACTION_ID>  &lt;PARENT_ID>1200003003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>INSURANCE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>quotationList.sprg?screenId=1200003007&amp;amp;screenName=INSURANCE&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>4&lt;/SEQUENCE>  &lt;SCREEN_NAME>INSURANCE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>insuranceQuotationDetails.sprg?screenId=1200003014&amp;amp;screenName=INSURANCE&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>4&lt;/SEQ_NO>  &lt;PATH>-112-223-34&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003008&lt;/ACTION_ID>  &lt;PARENT_ID>1200003003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CASHFLOW&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>quotationList.sprg?screenId=1200003008&amp;amp;screenName=CASHFLOW&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>5&lt;/SEQUENCE>  &lt;SCREEN_NAME>CASHFLOW&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>quotationCashflowDetail.sprg?screenId=1200003015&amp;amp;screenName=CASHFLOW&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>5&lt;/SEQ_NO>  &lt;PATH>-112-223-35&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003021&lt;/ACTION_ID>  &lt;PARENT_ID>1200003003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>REGISTRATION DETAILS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>quotationList.sprg?screenId=1200003021&amp;amp;screenName=REGISTRATION DETAILS&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>6&lt;/SEQUENCE>  &lt;SCREEN_NAME>REGISTRATION DETAILS&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>quotationVroDetail.sprg?&amp;amp;screenId=1200003022&amp;amp;screenName=REGISTRATION DETAILS&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>6&lt;/SEQ_NO>  &lt;PATH>-112-223-36&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003009&lt;/ACTION_ID>  &lt;PARENT_ID>1200003003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>ACCEPTANCE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>quotationList.sprg?screenId=1200003009&amp;amp;screenName=ACCEPTANCE&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>7&lt;/SEQUENCE>  &lt;SCREEN_NAME>ACCEPTANCE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>acceptanceDetails.sprg?screenId=1200003016&amp;amp;screenName=ACCEPTANCE&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>7&lt;/SEQ_NO>  &lt;PATH>-112-223-37&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003010&lt;/ACTION_ID>  &lt;PARENT_ID>1200003003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>ACQUISITION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>quotationList.sprg?screenId=1200003010&amp;amp;screenName=ACQUISITION&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>8&lt;/SEQUENCE>  &lt;SCREEN_NAME>ACQUISITION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>acquisitionDetails.sprg?screenId=1200003017&amp;amp;screenName=ACQUISITION&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>8&lt;/SEQ_NO>  &lt;PATH>-112-223-38&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003020&lt;/ACTION_ID>  &lt;PARENT_ID>1200003000&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>quotationList.sprg?screenId=1200003020&amp;amp;screenName=VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>24&lt;/SEQUENCE>  &lt;SCREEN_NAME>VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>quotationDetails.sprg?screenId=1200003013&amp;amp;screenName=QUOTATION&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>24&lt;/SEQ_NO>  &lt;PATH>-112-224&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004000&lt;/ACTION_ID>  &lt;PARENT_ID/>  &lt;DISPLAY_NAME>DM QUOTATION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>13&lt;/SEQUENCE>  &lt;SCREEN_NAME>DM_LOS_LEASE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>13&lt;/SEQ_NO>  &lt;PATH>-113&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004001&lt;/ACTION_ID>  &lt;PARENT_ID>1200004000&lt;/PARENT_ID>  &lt;DISPLAY_NAME>NEW &lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>NEW_DM_APPLICATION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-113-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004002&lt;/ACTION_ID>  &lt;PARENT_ID>1200004001&lt;/PARENT_ID>  &lt;DISPLAY_NAME>DM APPLICATION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>dMQuotationApplicantDetail.sprg?screenId=1200003002&amp;amp;screenName=NEW DM APPLICATION&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>2&lt;/SEQUENCE>  &lt;SCREEN_NAME>DM_APPLICATION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>dMQuotationApplicantDetail.sprg&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>newProspect&lt;/REQUEST_PAGE>  &lt;SEQ_NO>2&lt;/SEQ_NO>  &lt;PATH>-113-21-32&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004003&lt;/ACTION_ID>  &lt;PARENT_ID>1200004000&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PENDING ACTIVITIES&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>2&lt;/SEQUENCE>  &lt;SCREEN_NAME>PENDING ACTIVITES&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>2&lt;/SEQ_NO>  &lt;PATH>-113-22&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004005&lt;/ACTION_ID>  &lt;PARENT_ID>1200004003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>ASSET DETAILS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>dmProspectList.sprg?screenId=1200004005&amp;amp;screenName=ASSET&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>2&lt;/SEQUENCE>  &lt;SCREEN_NAME>ASSET&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>assetDetails.sprg?screenId=1200004019&amp;amp;screenName=ASSET&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>2&lt;/SEQ_NO>  &lt;PATH>-113-22-32&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004006&lt;/ACTION_ID>  &lt;PARENT_ID>1200004003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>QUOTATION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>dmProspectList.sprg?screenId=1200004006&amp;amp;screenName=QUOTATION&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>QUOTATION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>quotationDetails.sprg?screenId=1200004020&amp;amp;screenName=QUOTATION&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-113-22-33&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004008&lt;/ACTION_ID>  &lt;PARENT_ID>1200004003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CASHFLOW&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>dmProspectList.sprg?screenId=1200004008&amp;amp;screenName=CASHFLOW&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>5&lt;/SEQUENCE>  &lt;SCREEN_NAME>CASHFLOW&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>quotationCashflowDetail.sprg?screenId=1200004022&amp;amp;screenName=CASHFLOW&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>5&lt;/SEQ_NO>  &lt;PATH>-113-22-35&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004013&lt;/ACTION_ID>  &lt;PARENT_ID>1200004003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>DM OFFLINE ACTION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>dmProspectList.sprg?screenId=1200004013&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>6&lt;/SEQUENCE>  &lt;SCREEN_NAME>DM OFFLINE ACTION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>dmWorkflow.sprg?screenId=1200004016&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>6&lt;/SEQ_NO>  &lt;PATH>-113-22-36&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004009&lt;/ACTION_ID>  &lt;PARENT_ID>1200004003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>DM SANCTION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>dmProspectList.sprg?screenId=1200004009&amp;amp;screenName=DM SANCTION&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>6&lt;/SEQUENCE>  &lt;SCREEN_NAME>DM SANCTION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>creditDecision.do?actionPerformed=displayCreditDecisionPage&amp;amp;screenId=1000000086&amp;amp;screenName=DM SANCTION&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>6&lt;/SEQ_NO>  &lt;PATH>-113-22-36&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004010&lt;/ACTION_ID>  &lt;PARENT_ID>1200004003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CAPITALISATION  MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>dmProspectList.sprg?screenId=1200004010&amp;amp;screenName=DISBURSAL_MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>7&lt;/SEQUENCE>  &lt;SCREEN_NAME>CAPITALISATION  MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>disbursalFrmNav.do?actionPerformed=displayDMDisbursalInfo&amp;amp;screenId=1200004017&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>7&lt;/SEQ_NO>  &lt;PATH>-113-22-37&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004023&lt;/ACTION_ID>  &lt;PARENT_ID>1200004000&lt;/PARENT_ID>  &lt;DISPLAY_NAME>DM CANCELLATION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LOAN CANCELATION&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>DM CANCELLATION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-113-23&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004012&lt;/ACTION_ID>  &lt;PARENT_ID>1200004000&lt;/PARENT_ID>  &lt;DISPLAY_NAME>DM VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>dmProspectList.sprg?screenId=1200004012&amp;amp;screenName=VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>DM VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>quotationDetails.sprg?screenId=1200004020&amp;amp;screenName=QUOTATION&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-113-23&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004024&lt;/ACTION_ID>  &lt;PARENT_ID>1200004023&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>dmProspectList.sprg?screenId=1200004024&amp;amp;screenName=CANCELLATION MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LOAN CANCELATION&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>DM CANCELLATION MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>disbCancellationAction.do?actionPerformed=displayDmCancellation&amp;amp;screenId=1200004027&amp;amp;screenName=CANCELLATION MAKER&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-113-23-31&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004026&lt;/ACTION_ID>  &lt;PARENT_ID>1200004023&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>dmProspectList.sprg?screenId=1200004026&amp;amp;screenName=CANCELLATION VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LOAN CANCELATION&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>DM CANCELLATION VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>disbCancellationAction.do?actionPerformed=displayDmCancellation&amp;amp;screenId=1200004026&amp;amp;screenName=CANCELLATION VIEWER&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-113-23-33&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004029&lt;/ACTION_ID>  &lt;PARENT_ID>1200004000&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PAYMENT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PAYMENT&lt;/MENU_TYPE>  &lt;SEQUENCE>4&lt;/SEQUENCE>  &lt;SCREEN_NAME>PAYMENT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>4&lt;/SEQ_NO>  &lt;PATH>-113-24&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004030&lt;/ACTION_ID>  &lt;PARENT_ID>1200004029&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>forwardFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1200004030&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PAYMENT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>PAYMENT MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>paymentAction.do?actionPerformed=displayPaymentScreen&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-113-24-31&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004032&lt;/ACTION_ID>  &lt;PARENT_ID>1200004029&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>forwardFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1200004032&amp;amp;screenName=VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PAYMENT&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>PAYMENT VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>paymentAction.do?actionPerformed=displayPaymentScreen&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-113-24-33&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000017&lt;/ACTION_ID>  &lt;PARENT_ID/>  &lt;DISPLAY_NAME>LMS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LMS&lt;/MENU_TYPE>  &lt;SEQUENCE>14&lt;/SEQUENCE>  &lt;SCREEN_NAME>LMS&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>14&lt;/SEQ_NO>  &lt;PATH>-114&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106854&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>INVOICE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>INVOICE&lt;/MENU_TYPE>  &lt;SEQUENCE>31&lt;/SEQUENCE>  &lt;SCREEN_NAME>INVOICE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>31&lt;/SEQ_NO>  &lt;PATH>-114-231&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106856&lt;/ACTION_ID>  &lt;PARENT_ID>1200106854&lt;/PARENT_ID>  &lt;DISPLAY_NAME>INVOICE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>forwardFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1200106856&amp;amp;screenName=INVOICE&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>INVOICE&lt;/MENU_TYPE>  &lt;SEQUENCE>50&lt;/SEQUENCE>  &lt;SCREEN_NAME>INVOICE&lt;/SCREEN_NAME>  &lt;AUTHMODE/>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>invoiceDetails.sprg?screenId=1200106856&amp;amp;screenName=INVOICE&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>50&lt;/SEQ_NO>  &lt;PATH>-114-231-350&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106858&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>DM PDE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>DM PDE&lt;/MENU_TYPE>  &lt;SEQUENCE>32&lt;/SEQUENCE>  &lt;SCREEN_NAME>DM PDE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>32&lt;/SEQ_NO>  &lt;PATH>-114-232&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106859&lt;/ACTION_ID>  &lt;PARENT_ID>1200106858&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1200106859&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>DM PDE&lt;/MENU_TYPE>  &lt;SEQUENCE>33&lt;/SEQUENCE>  &lt;SCREEN_NAME>DM PDE MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>pdeApplicantList.do?actionPerformed=displayPDECustomerList&amp;amp;screenId=1000000090&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>33&lt;/SEQ_NO>  &lt;PATH>-114-232-333&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000034&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>RECEIPT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>OTC&lt;/MENU_TYPE>  &lt;SEQUENCE>42&lt;/SEQUENCE>  &lt;SCREEN_NAME>OTC&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>42&lt;/SEQ_NO>  &lt;PATH>-114-242&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000036&lt;/ACTION_ID>  &lt;PARENT_ID>1000000034&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000036&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>OTC&lt;/MENU_TYPE>  &lt;SEQUENCE>44&lt;/SEQUENCE>  &lt;SCREEN_NAME>RECEIPT MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>otcFrmNavAction.do?actionPerformed=displayOtcInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/lms/otcFrmNavAction.do?actionPerformed=displayOtcInfo&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>44&lt;/SEQ_NO>  &lt;PATH>-114-242-344&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000038&lt;/ACTION_ID>  &lt;PARENT_ID>1000000034&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PAYMENT HISTORY&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000038&amp;amp;screenName=VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>OTC&lt;/MENU_TYPE>  &lt;SEQUENCE>46&lt;/SEQUENCE>  &lt;SCREEN_NAME>PAYMENT HISTORY&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>viewerAction.do?actionPerformed=displayViewerInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>otcFrmNavAction.do?actionPerformed=displayOtcInfo&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>46&lt;/SEQ_NO>  &lt;PATH>-114-242-346&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000010034&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>RECEIPT CANCELLATION &lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>RECEIPT_CANCELLATION &lt;/MENU_TYPE>  &lt;SEQUENCE>44&lt;/SEQUENCE>  &lt;SCREEN_NAME>RECEIPT_CANCELLATION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>44&lt;/SEQ_NO>  &lt;PATH>-114-244&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000010035&lt;/ACTION_ID>  &lt;PARENT_ID>1000010034&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000010035&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>RECEIPT_CANCELLATION &lt;/MENU_TYPE>  &lt;SEQUENCE>44&lt;/SEQUENCE>  &lt;SCREEN_NAME>RECEIPT CANCELLATION MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>receipt_cancellationFrmNavAction.do?actionPerformed=displayreceiptInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/lms/otcFrmNavAction.do?actionPerformed=displayOtcInfo&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>44&lt;/SEQ_NO>  &lt;PATH>-114-244-344&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000010109&lt;/ACTION_ID>  &lt;PARENT_ID>1000010034&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000010109&amp;amp;screenName=VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>RECEIPT CANCELLATION&lt;/MENU_TYPE>  &lt;SEQUENCE>46&lt;/SEQUENCE>  &lt;SCREEN_NAME>RECEIPT CANCELLATION VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>viewerAction.do?actionPerformed=displayViewerInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>otcFrmNavAction.do?actionPerformed=displayOtcInfo&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>46&lt;/SEQ_NO>  &lt;PATH>-114-244-346&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000039&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>REFUND&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REFUND&lt;/MENU_TYPE>  &lt;SEQUENCE>47&lt;/SEQUENCE>  &lt;SCREEN_NAME>REFUND&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>47&lt;/SEQ_NO>  &lt;PATH>-114-247&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000041&lt;/ACTION_ID>  &lt;PARENT_ID>1000000039&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000041&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REFUND&lt;/MENU_TYPE>  &lt;SEQUENCE>49&lt;/SEQUENCE>  &lt;SCREEN_NAME>REFUND MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>refundFrmNavAction.do?actionPerformed=displayRefundInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>49&lt;/SEQ_NO>  &lt;PATH>-114-247-349&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000043&lt;/ACTION_ID>  &lt;PARENT_ID>1000000039&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000043&amp;amp;screenName=VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REFUND&lt;/MENU_TYPE>  &lt;SEQUENCE>51&lt;/SEQUENCE>  &lt;SCREEN_NAME>REFUND VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>viewerAction.do?actionPerformed=displayViewerInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>51&lt;/SEQ_NO>  &lt;PATH>-114-247-351&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001980&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MANUAL ADVICE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>MANUAL ADVICE&lt;/MENU_TYPE>  &lt;SEQUENCE>48&lt;/SEQUENCE>  &lt;SCREEN_NAME>MANUAL ADVICE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>48&lt;/SEQ_NO>  &lt;PATH>-114-248&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001981&lt;/ACTION_ID>  &lt;PARENT_ID>1000001980&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000001981&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>MANUAL ADVICE&lt;/MENU_TYPE>  &lt;SEQUENCE>10&lt;/SEQUENCE>  &lt;SCREEN_NAME>MANUAL ADVICE MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>manualAdviceAction.do?actionPerformed=displayManualAdviceInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/lms/manualAdviceFrmNavAction.do?actionPerformed=displayManualAdviceInfo&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>10&lt;/SEQ_NO>  &lt;PATH>-114-248-310&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000054&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>KNOCK OFF&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>KNOCK OFF&lt;/MENU_TYPE>  &lt;SEQUENCE>57&lt;/SEQUENCE>  &lt;SCREEN_NAME>KNOCK OFF&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>57&lt;/SEQ_NO>  &lt;PATH>-114-257&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000056&lt;/ACTION_ID>  &lt;PARENT_ID>1000000054&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000056&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>KNOCK OFF&lt;/MENU_TYPE>  &lt;SEQUENCE>59&lt;/SEQUENCE>  &lt;SCREEN_NAME>KNOCK OFF MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>knockOffFrmNavAction.do?actionPerformed=displayKnockOffInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>59&lt;/SEQ_NO>  &lt;PATH>-114-257-359&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000058&lt;/ACTION_ID>  &lt;PARENT_ID>1000000054&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000058&amp;amp;screenName=VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>KNOCK OFF&lt;/MENU_TYPE>  &lt;SEQUENCE>61&lt;/SEQUENCE>  &lt;SCREEN_NAME>KNOCK OFF VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>viewerAction.do?actionPerformed=displayViewerInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>knockOffFrmNavAction.do?actionPerformed=displayKnockOffInfo&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>61&lt;/SEQ_NO>  &lt;PATH>-114-257-361&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000024&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>INSTR MANAGEMENT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>INSTRUMENT MANAGEMENT&lt;/MENU_TYPE>  &lt;SEQUENCE>59&lt;/SEQUENCE>  &lt;SCREEN_NAME>INSTRUMENT MANAGEMENT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>59&lt;/SEQ_NO>  &lt;PATH>-114-259&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000025&lt;/ACTION_ID>  &lt;PARENT_ID>1000000024&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PDC/ECS GENERATE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3_1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PDC_GENERATE&lt;/MENU_TYPE>  &lt;SEQUENCE>33&lt;/SEQUENCE>  &lt;SCREEN_NAME>PDC/ECS GENERATE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>33&lt;/SEQ_NO>  &lt;PATH>-114-259-333&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000026&lt;/ACTION_ID>  &lt;PARENT_ID>1000000025&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000026&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PDC_GENERATE&lt;/MENU_TYPE>  &lt;SEQUENCE>35&lt;/SEQUENCE>  &lt;SCREEN_NAME>PDC/ECS GENERATE MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>pdcEcsFrmNavAction.do?actionPerformed=displayPdcEcsInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/lms/pdcEcsFrmNavAction.do?actionPerformed=displayPdcEcsInfo&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>35&lt;/SEQ_NO>  &lt;PATH>-114-259-333-435&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000029&lt;/ACTION_ID>  &lt;PARENT_ID>1000000024&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PDC/ECS EDIT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3_1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PDC_EDIT&lt;/MENU_TYPE>  &lt;SEQUENCE>37&lt;/SEQUENCE>  &lt;SCREEN_NAME>PDC/ECS EDIT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>37&lt;/SEQ_NO>  &lt;PATH>-114-259-337&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000031&lt;/ACTION_ID>  &lt;PARENT_ID>1000000029&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000031&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PDC_EDIT&lt;/MENU_TYPE>  &lt;SEQUENCE>39&lt;/SEQUENCE>  &lt;SCREEN_NAME>PDC/ECS EDIT MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>pdcEcsEditFrmNavAction.do?actionPerformed=displayPdcEcsEditInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/lms/pdcEcsEditFrmNavAction.do?actionPerformed=displayPdcEcsEditInfo&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>39&lt;/SEQ_NO>  &lt;PATH>-114-259-337-439&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000033&lt;/ACTION_ID>  &lt;PARENT_ID>1000000029&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000033&amp;amp;screenName=VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PDC_EDIT&lt;/MENU_TYPE>  &lt;SEQUENCE>41&lt;/SEQUENCE>  &lt;SCREEN_NAME>PDC/ECS EDIT VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>viewerAction.do?actionPerformed=displayViewerInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>pdcEcsEditFrmNavAction.do?actionPerformed=displayPdcEcsEditInfo&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>41&lt;/SEQ_NO>  &lt;PATH>-114-259-337-441&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000049&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>WAIVE OFF&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>WAIVEOFF&lt;/MENU_TYPE>  &lt;SEQUENCE>60&lt;/SEQUENCE>  &lt;SCREEN_NAME>WAIVEOFF&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>60&lt;/SEQ_NO>  &lt;PATH>-114-260&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000051&lt;/ACTION_ID>  &lt;PARENT_ID>1000000049&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000051&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>WAIVE&lt;/MENU_TYPE>  &lt;SEQUENCE>54&lt;/SEQUENCE>  &lt;SCREEN_NAME>WAIVE MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>waiveOffFrmNavAction.do?actionPerformed=displayWaiveOffInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>54&lt;/SEQ_NO>  &lt;PATH>-114-260-354&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000053&lt;/ACTION_ID>  &lt;PARENT_ID>1000000049&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000053&amp;amp;screenName=VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>WAIVE&lt;/MENU_TYPE>  &lt;SEQUENCE>56&lt;/SEQUENCE>  &lt;SCREEN_NAME>WAIVE VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>viewerAction.do?actionPerformed=displayViewerInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>waiveOffFrmNavAction.do?actionPerformed=displayWaiveOffInfo&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>56&lt;/SEQ_NO>  &lt;PATH>-114-260-356&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000059&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CLOSE / FORECLOSE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>FORECLOSURE&lt;/MENU_TYPE>  &lt;SEQUENCE>62&lt;/SEQUENCE>  &lt;SCREEN_NAME>CLOSE/FORECLOSE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>62&lt;/SEQ_NO>  &lt;PATH>-114-262&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000061&lt;/ACTION_ID>  &lt;PARENT_ID>1000000059&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000061&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>FORECLOSURE&lt;/MENU_TYPE>  &lt;SEQUENCE>64&lt;/SEQUENCE>  &lt;SCREEN_NAME>CLOSE/FORECLOSE MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>foreClouserFrmNavAction.do?actionPerformed=displayForeClouserInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>64&lt;/SEQ_NO>  &lt;PATH>-114-262-364&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000063&lt;/ACTION_ID>  &lt;PARENT_ID>1000000059&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000063&amp;amp;screenName=VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>FORECLOSURE&lt;/MENU_TYPE>  &lt;SEQUENCE>66&lt;/SEQUENCE>  &lt;SCREEN_NAME>CLOSE/FORECLOSE VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>foreClouserFrmNavAction.do?actionPerformed=displayForeClouserInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>66&lt;/SEQ_NO>  &lt;PATH>-114-262-366&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000064&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BATCH PROCESS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH PROCESS&lt;/MENU_TYPE>  &lt;SEQUENCE>67&lt;/SEQUENCE>  &lt;SCREEN_NAME>BATCH PROCESS&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>67&lt;/SEQ_NO>  &lt;PATH>-114-267&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000065&lt;/ACTION_ID>  &lt;PARENT_ID>1000000064&lt;/PARENT_ID>  &lt;DISPLAY_NAME>GENERATION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>forwardFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000065&amp;amp;screenName=GENERATION&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3_1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_GENERATION&lt;/MENU_TYPE>  &lt;SEQUENCE>68&lt;/SEQUENCE>  &lt;SCREEN_NAME>GENERATION&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>batchGenerationFrmNavAction.do?actionPerformed=displayBatchProcessInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>68&lt;/SEQ_NO>  &lt;PATH>-114-267-368&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001045&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>RESCHEDULING CASE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>RESCHEDULE  CASE&lt;/MENU_TYPE>  &lt;SEQUENCE>82&lt;/SEQUENCE>  &lt;SCREEN_NAME>RESCHEDULE CASE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>82&lt;/SEQ_NO>  &lt;PATH>-114-282&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001046&lt;/ACTION_ID>  &lt;PARENT_ID>1000001045&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000001046&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>RESCHEDULE  CASE&lt;/MENU_TYPE>  &lt;SEQUENCE>83&lt;/SEQUENCE>  &lt;SCREEN_NAME>RESCHEDULE CASE MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>rescheduleCaseFrmNavAction.do?actionPerformed=displayRescheduleCaseData&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>83&lt;/SEQ_NO>  &lt;PATH>-114-282-383&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000074&lt;/ACTION_ID>  &lt;PARENT_ID/>  &lt;DISPLAY_NAME>REPORTS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>17&lt;/SEQUENCE>  &lt;SCREEN_NAME>REPORTS&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>17&lt;/SEQ_NO>  &lt;PATH>-117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000010249&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>ACCOUNTING REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1000010249&amp;amp;actionId=1000010249&amp;amp;mode=R&amp;amp;actionName=ACCOUNTING_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>ACCOUNTING_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>ACCOUNTING_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1000010249\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109018&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>ACCRUAL REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109018&amp;amp;actionId=1200109018&amp;amp;mode=R&amp;amp;actionName=ACCRUAL_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>ACCRUAL_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>ACCRUAL REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109018\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109019&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>ASSET RECEIPT REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109019&amp;amp;actionId=1200109019&amp;amp;mode=R&amp;amp;actionName=ASSET_RECEIPT_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>ASSET_RECEIPT_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>ASSET RECEIPT REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109019\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106909&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>AUTO RESCHEDULE STATUS REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1100106909&amp;amp;actionId=1100106909&amp;amp;mode=R&amp;amp;actionName=AUTO_RESCHEDULE_STATUS_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>AUTO_RESCHEDULE_STATUS_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>AUTO_RESCHEDULE_STATUS_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1100106909\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106908&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BASE_RATE_CHANGE_TRACK_REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1100106908&amp;amp;actionId=1100106908&amp;amp;mode=R&amp;amp;actionName=BASE_RATE_CHANGE_TRACK_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BASE_RATE_CHANGE_TRACK_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>BASE_RATE_CHANGE_TRACK_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1100106908\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109049&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CLIENT MIS REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109049&amp;amp;actionId=1200109049&amp;amp;mode=R&amp;amp;actionName=CLIENT_MIS_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>CLIENT MIS REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109049\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109103&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER DETAIL REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109103&amp;amp;actionId=1200109103&amp;amp;mode=R&amp;amp;actionName=CUSTOMER_DETAIL_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>CUSTOMER DETAIL REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER DETAIL REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109103\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109098&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>DOCUMENT PENDING AND EXPIRY REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109098&amp;amp;actionId=1200109098&amp;amp;mode=R&amp;amp;actionName=DOCUMENT_EXPIRY&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>DOCUMENT EXPIRY&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>DOCUMENT PENDING AND EXPIRY REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109097\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109124&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>INSURANCE REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109124&amp;amp;actionId=1200109124&amp;amp;mode=R&amp;amp;actionName=INSURANCE_EXPIRY_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>INSURANCE REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109124\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109010&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>INVOICE REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109010&amp;amp;actionId=1200109010&amp;amp;mode=R&amp;amp;actionName=INVOICE_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>INVOICE_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>INVOICE REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109010\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109090&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEAD FOLLOW UP&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109090&amp;amp;actionId=1200109090&amp;amp;mode=R&amp;amp;actionName=LEAD_FOLLOW_UP&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LEAD FOLLOW UP&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEAD FOLLOW UP&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109090\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109037&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEAD STATUS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109037&amp;amp;actionId=1200109037&amp;amp;mode=R&amp;amp;actionName=LEAD_STATUS&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LEAD STATUS&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEAD STATUS&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109037\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109169&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE REGISTRATION TRACKING&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109169&amp;amp;actionId=1200109169&amp;amp;mode=R&amp;amp;actionName=LEASE_REGISTRATION_TRACKING&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LEASE REGISTRATION TRACKING&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE REGISTRATION TRACKING&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109169\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109097&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LIMIT EXPIRY&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109097&amp;amp;actionId=1200109097&amp;amp;mode=R&amp;amp;actionName=LIMIT_EXPIRY&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMIT EXPIRY&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>LIMIT EXPIRY&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109097\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109128&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LPI REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109128&amp;amp;actionId=1200109128&amp;amp;mode=R&amp;amp;actionName=LPI REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>LPI REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109128\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109170&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MATURING AGREEMENTS REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109170&amp;amp;actionId=1200109170&amp;amp;mode=R&amp;amp;actionName=MATURING_AGREEMENTS_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>MATURING AGREEMENTS REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>MATURING AGREEMENTS&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109170\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109030&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>OUTSTANDING DUE REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109030&amp;amp;actionId=1200109030&amp;amp;mode=R&amp;amp;actionName=OUTSTANDING_DUE_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>OUTSTANDING_DUE_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>OUTSTANDING DUE REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109030\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109029&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>RV DUE REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109029&amp;amp;actionId=1200109029&amp;amp;mode=R&amp;amp;actionName=RV_DUE_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>RV_DUE_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>RV DUE REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109029\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109062&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>SCHEDULED REPORTS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109062&amp;amp;actionId=1200109062&amp;amp;mode=R&amp;amp;actionName=SCHEDULED_REPORTS&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>SCHEDULED REPORTS&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109062\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109017&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>TRANSACTION REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109017&amp;amp;actionId=1200109017&amp;amp;mode=R&amp;amp;actionName=TRANSACTION_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>TRANSACTION_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>TRANSACTION REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109017\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109125&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>WAIVER_REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109125&amp;amp;actionId=1200109125&amp;amp;mode=R&amp;amp;actionName=WAIVER_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>WAIVER_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109125\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108937&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MANUAL ADVICE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108937&amp;amp;actionId=1200108937&amp;amp;mode=R&amp;amp;actionName=MANUAL_ADVICE&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>MANUAL_ADVICE&lt;/MENU_TYPE>  &lt;SEQUENCE>105&lt;/SEQUENCE>  &lt;SCREEN_NAME>MANUAL_ADVICE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200108937\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>105&lt;/SEQ_NO>  &lt;PATH>-117-2105&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109133&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE PROPOSAL&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109133&amp;amp;actionId=1200109133&amp;amp;mode=R&amp;amp;actionName=LEASE_PROPOSAL&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_PROPOSAL&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109133&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-2117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106732&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PO REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200106732&amp;amp;actionId=1200106732&amp;amp;mode=R&amp;amp;actionName=PO_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PO_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>PO_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200106732&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-2117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108995&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>QUOTATION DETAIL REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108995&amp;amp;actionId=1200108995&amp;amp;mode=R&amp;amp;actionName=QUOTATION_DETAIL_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION_DETAIL_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>121&lt;/SEQUENCE>  &lt;SCREEN_NAME>QUOTATION DETAIL REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200108995\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>121&lt;/SEQ_NO>  &lt;PATH>-117-2121&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100010146&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>RESCHEDULING REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1100010146&amp;amp;actionId=1100010146&amp;amp;mode=R&amp;amp;actionName=RESCHEDULING_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>RESCHEDULING_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>129&lt;/SEQUENCE>  &lt;SCREEN_NAME>RESCHEDULING_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1100010146\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>129&lt;/SEQ_NO>  &lt;PATH>-117-2129&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106857&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BATCH DTL REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200106857&amp;amp;actionId=1200106857&amp;amp;mode=R&amp;amp;actionName=BATCH_DTL_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_DTL_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>13&lt;/SEQUENCE>  &lt;SCREEN_NAME>BATCH_DTL_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200106857\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>13&lt;/SEQ_NO>  &lt;PATH>-117-213&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106489&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>STATEMENT OF ACCOUNT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1100106489&amp;amp;actionId=1100106489&amp;amp;mode=R&amp;amp;actionName=SOA_NEW&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>137&lt;/SEQUENCE>  &lt;SCREEN_NAME>STATEMENT_OF_ACCOUNT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1100106489\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>137&lt;/SEQ_NO>  &lt;PATH>-117-2137&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106727&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BATCH PRESENTATION REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1100106727&amp;amp;actionId=1100106727&amp;amp;mode=R&amp;amp;actionName=BATCH_PRESENTATION_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_PRESENTATION_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>14&lt;/SEQUENCE>  &lt;SCREEN_NAME>BATCH_PRESENTATION_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1100106727\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>14&lt;/SEQ_NO>  &lt;PATH>-117-214&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106510&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>TRANSACTION REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1100106510&amp;amp;actionId=1100106510&amp;amp;mode=R&amp;amp;actionName=TRANSACTION_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>TRANSACTION_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>140&lt;/SEQUENCE>  &lt;SCREEN_NAME>TRANSACTION REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1100106510\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>140&lt;/SEQ_NO>  &lt;PATH>-117-2140&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106511&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>TRIAL BALANCE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1100106511&amp;amp;actionId=1100106511&amp;amp;mode=R&amp;amp;actionName=TRIAL_BALANCE&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>TRIAL_BALANCE&lt;/MENU_TYPE>  &lt;SEQUENCE>141&lt;/SEQUENCE>  &lt;SCREEN_NAME>TRIAL BALANCE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1100106511\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>141&lt;/SEQ_NO>  &lt;PATH>-117-2141&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108988&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VENDOR PAYMENT REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108988&amp;amp;actionId=1200108988&amp;amp;mode=R&amp;amp;actionName=VENDOR_PAYMENT_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>VENDOR_PAYMENT_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>145&lt;/SEQUENCE>  &lt;SCREEN_NAME>VENDOR PAYMENT REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200108988\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>145&lt;/SEQ_NO>  &lt;PATH>-117-2145&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106729&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BATCH STATUS REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1100106729&amp;amp;actionId=1100106729&amp;amp;mode=R&amp;amp;actionName=BATCH_STATUS_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_STATUS_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>15&lt;/SEQUENCE>  &lt;SCREEN_NAME>BATCH_STATUS_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1100106729\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>15&lt;/SEQ_NO>  &lt;PATH>-117-215&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108922&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BATCH UPLOAD ISSUES REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108922&amp;amp;actionId=1200108922&amp;amp;mode=R&amp;amp;actionName=BATCH_UPLOAD_ISSUES_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD_ISSUES_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>16&lt;/SEQUENCE>  &lt;SCREEN_NAME>BATCH_UPLOAD_ISSUES_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;=1200108922&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>16&lt;/SEQ_NO>  &lt;PATH>-117-216&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109108&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CREDIT OPERATION REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>17&lt;/SEQUENCE>  &lt;SCREEN_NAME>CREDIT_OPERATION_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>17&lt;/SEQ_NO>  &lt;PATH>-117-217&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109132&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>ANNEXES OL&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109132&amp;amp;actionId=1200109132&amp;amp;mode=R&amp;amp;actionName=ANNEXES_OL&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>ANNEXES_OL&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109132&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109137&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>AUDIT CONFIRMATION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109137&amp;amp;actionId=1200109137&amp;amp;mode=R&amp;amp;actionName=AUDIT_CONFIRMATION&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>AUDIT_CONFIRMATION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109137&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109121&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BOARD OF RESOLUTION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109121&amp;amp;actionId=1200109121&amp;amp;mode=R&amp;amp;actionName=BOARD_OF_RESOLUTION&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>BOARD_OF_RESOLUTION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109121&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109112&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BOARD RESOLUTION CORPORATE GUARANTOR&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109112&amp;amp;actionId=1200109112&amp;amp;mode=R&amp;amp;actionName=BOARD_RESOLUTION_CORPORATE_GUARANTOR&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>BOARD_RESOLUTION_CORPORATE_GUARANTOR&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109112&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109136&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CANCELLATION SALARY DEDUCTION FORM&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109136&amp;amp;actionId=1200109136&amp;amp;mode=R&amp;amp;actionName=CANCELLATION_SALARY_DEDUCTION_FORM&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>CANCELLATION_SALARY_DEDUCTION_FORM&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109136&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109135&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CANCELLATION STANDING ORDER FORM&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109135&amp;amp;actionId=1200109135&amp;amp;mode=R&amp;amp;actionName=CANCELLATION_STANDING_ORDER_FORM&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>CANCELLATION_STANDING_ORDER_FORM&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109135&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109119&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CERTIFICAT DE GAGE SECOND HAND&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109119&amp;amp;actionId=1200109119&amp;amp;mode=R&amp;amp;actionName=CERTIFICAT_DE_GAGE_SECOND_HAND&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>CERTIFICAT_DE_GAGE_SECOND_HAND&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109119&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109113&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CHANGE IN ENGINE NUMBER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109113&amp;amp;actionId=1200109113&amp;amp;mode=R&amp;amp;actionName=CHANGE_IN_ENGINE_NUMBER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>CHANGE_IN_ENGINE_NUMBER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109113&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109114&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CHANGE IN REGISTRATION NUMBER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109114&amp;amp;actionId=1200109114&amp;amp;mode=R&amp;amp;actionName=CHANGE_IN_REGISTRATION_NUMBER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>CHANGE_IN_REGISTRATION_NUMBER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109114&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109115&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CHANGE OF NAME OF HORSEPOWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109115&amp;amp;actionId=1200109115&amp;amp;mode=R&amp;amp;actionName=CHANGE_OF_NAME_OF_HORSEPOWER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>CHANGE_OF_NAME_OF_HORSEPOWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109115&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109116&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CORPORATE GUARANTEE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109116&amp;amp;actionId=1200109116&amp;amp;mode=R&amp;amp;actionName=CORPORATE_GUARANTEE&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>CORPORATE_GUARANTEE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109116&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109155&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>COVER LETTER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109155&amp;amp;actionId=1200109155&amp;amp;mode=R&amp;amp;actionName=COVER_LETTER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>COVER_LETTER&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109155&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109126&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>DIRECT DEBIT FORM&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109126&amp;amp;actionId=1200109126&amp;amp;mode=R&amp;amp;actionName=DIRECT_DEBIT_FORM&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>DIRECT_DEBIT_FORM&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109126&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109117&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>HORSEPOWER FULL&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109117&amp;amp;actionId=1200109117&amp;amp;mode=R&amp;amp;actionName=HORSEPOWER_FULL&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>HORSEPOWER_FULL&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109117&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109143&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE AGREEMENT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109143&amp;amp;actionId=1200109143&amp;amp;mode=R&amp;amp;actionName=LEASE_AGREEMENT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_AGREEMENT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109143&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109131&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE AGREEMENT OL&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109131&amp;amp;actionId=1200109131&amp;amp;mode=R&amp;amp;actionName=LEASE_AGREEMENT_OL&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_AGREEMENT_OL&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109131&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109111&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LETTER OF UNDERTAKING&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109111&amp;amp;actionId=1200109111&amp;amp;mode=R&amp;amp;actionName=LETTER_OF_UNDERTAKING&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>LETTER_OF_UNDERTAKING&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109111&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109118&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LOST HORSEPOWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109118&amp;amp;actionId=1200109118&amp;amp;mode=R&amp;amp;actionName=LOST_HORSEPOWER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>LOST_HORSEPOWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109118&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109120&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>NO LIABILITY LETTER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109120&amp;amp;actionId=1200109120&amp;amp;mode=R&amp;amp;actionName=NO_LIABILITY_LETTER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>NO_LIABILITY_LETTER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109120&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109141&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>OFFER LETTER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109141&amp;amp;actionId=1200109141&amp;amp;mode=R&amp;amp;actionName=OFFER_LETTER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>OFFER_LETTER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109141&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109067&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>REPAYMENT SCHEDULE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109067&amp;amp;actionId=1200109067&amp;amp;mode=R&amp;amp;actionName=REPAYMENT_SCHEDULE&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPAYMENT_SCHEDULE&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>REPAYMENT_SCHEDULE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109067&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109109&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>RIGHT OF SET OFF LETTER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109109&amp;amp;actionId=1200109109&amp;amp;mode=R&amp;amp;actionName=RIGHT_OF_SET_OFF_LETTER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>RIGHT_OF_SET_OFF_LETTER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109109&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109140&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>RV LETTER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109140&amp;amp;actionId=1200109140&amp;amp;mode=R&amp;amp;actionName=RV_LETTER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>RV_LETTER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109140&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109139&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>RV SALES DEED AND CERTIFICATE OF GAGE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109139&amp;amp;actionId=1200109139&amp;amp;mode=R&amp;amp;actionName=RV_SALES_DEED_AND_CERTIFICATE_OF_GAGE&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>RV_SALES_DEED_AND_CERTIFICATE_OF_GAGE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109139&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109134&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>SALARY DEDUCTION FORM&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109134&amp;amp;actionId=1200109134&amp;amp;mode=R&amp;amp;actionName=SALARY_DEDUCTION_FORM&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>SALARY_DEDUCTION_FORM&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109134&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109138&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>SALES DEED&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109138&amp;amp;actionId=1200109138&amp;amp;mode=R&amp;amp;actionName=SALES_DEED&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>SALES_DEED&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109138&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109003&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>SETTLEMENT LETTER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109003&amp;amp;actionId=1200109003&amp;amp;mode=R&amp;amp;actionName=FORECLOSURE_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>FORECLOSURE_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109003&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109127&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>STANDING ORDER FORM&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109127&amp;amp;actionId=1200109127&amp;amp;mode=R&amp;amp;actionName=STANDING_ORDER_FORM&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>STANDING_ORDER_FORM&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109127&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109110&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>SUBORDINATION OF SHAREHOLDERS LOAN&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109110&amp;amp;actionId=1200109110&amp;amp;mode=R&amp;amp;actionName=SUBORDINATION_OF_SHAREHOLDERS_LOAN&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>SUBORDINATION_OF_SHAREHOLDERS_LOAN&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109110&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109142&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>SUPPLIER LETTER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109142&amp;amp;actionId=1200109142&amp;amp;mode=R&amp;amp;actionName=SUPPLIER_LETTER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>SUPPLIER_LETTER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109142&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108947&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BULK INVOICE REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>llmDashboard.do?actionPerformed=getDashboard&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BULK_INVOICE_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>20&lt;/SEQUENCE>  &lt;SCREEN_NAME>BULK_INVOICE_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>llmDashboard.do?actionPerformed=getDashboard&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>20&lt;/SEQ_NO>  &lt;PATH>-117-220&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106855&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>COVERNOTE REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>llmDashboard.do?actionPerformed=getDashboard&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>COVERNOTE_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>37&lt;/SEQUENCE>  &lt;SCREEN_NAME>COVERNOTE_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>llmDashboard.do?actionPerformed=getDashboard&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>37&lt;/SEQ_NO>  &lt;PATH>-117-237&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106539&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>AGEING REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1100106539&amp;amp;actionId=1100106539&amp;amp;mode=R&amp;amp;actionName=AGING_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>AGEING_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>4&lt;/SEQUENCE>  &lt;SCREEN_NAME>AGEING REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1100106539\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>4&lt;/SEQ_NO>  &lt;PATH>-117-24&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108999&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BRANCH WISE SUMMARY&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108999&amp;amp;actionId=1200108999&amp;amp;mode=R&amp;amp;actionName=BRANCH_WISE&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>40&lt;/SEQUENCE>  &lt;SCREEN_NAME>BRANCH WISE SUMMARY&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200108999\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>40&lt;/SEQ_NO>  &lt;PATH>-117-240&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109000&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>USER WISE SUMMARY&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109000&amp;amp;actionId=1200109000&amp;amp;mode=R&amp;amp;actionName=USER_WISE&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>40&lt;/SEQUENCE>  &lt;SCREEN_NAME>USER WISE SUMMARY&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109000\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>40&lt;/SEQ_NO>  &lt;PATH>-117-240&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108980&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER LIMIT REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108980&amp;amp;actionId=1200108980&amp;amp;mode=R&amp;amp;actionName=CUSTOMER_LIMIT_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>CUSTOMER_LIMIT_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>42&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER LIMIT REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200108980\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>42&lt;/SEQ_NO>  &lt;PATH>-117-242&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108996&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>FORECLOSURE REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108996&amp;amp;actionId=1200108996&amp;amp;mode=R&amp;amp;actionName=FORECLOSURE_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>FORECLOSURE_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>57&lt;/SEQUENCE>  &lt;SCREEN_NAME>FORECLOSURE REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200108996\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>57&lt;/SEQ_NO>  &lt;PATH>-117-257&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106710&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>ALCO REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1100106710&amp;amp;actionId=1100106710&amp;amp;mode=R&amp;amp;actionName=ALCO_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>ALCO_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>6&lt;/SEQUENCE>  &lt;SCREEN_NAME>ALCO_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1100106710\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>6&lt;/SEQ_NO>  &lt;PATH>-117-26&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108979&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>GROUP CUSTOMER REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108979&amp;amp;actionId=1200108979&amp;amp;mode=R&amp;amp;actionName=GROUP_CUSTOMER_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>GROUP_CUSTOMER_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>61&lt;/SEQUENCE>  &lt;SCREEN_NAME>GROUP CUSTOMER REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200108979\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>61&lt;/SEQ_NO>  &lt;PATH>-117-261&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108981&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>GROUP LIMIT REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108981&amp;amp;actionId=1200108981&amp;amp;mode=R&amp;amp;actionName=GROUP_LIMIT_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>GROUP_LIMIT_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>62&lt;/SEQUENCE>  &lt;SCREEN_NAME>GROUP LIMIT REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200108981\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>62&lt;/SEQ_NO>  &lt;PATH>-117-262&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108929&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE CAM REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108929&amp;amp;actionId=1200108929&amp;amp;mode=R&amp;amp;actionName=LEASE_CAM_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LEASE_CAM_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>90&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_CAM_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;=1200108929&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>90&lt;/SEQ_NO>  &lt;PATH>-117-290&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108906&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE CREDIT NOTE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108906&amp;amp;actionId=1200108906&amp;amp;mode=R&amp;amp;actionName=LEASE_CREDIT_NOTE&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LEASE_CREDIT_NOTE&lt;/MENU_TYPE>  &lt;SEQUENCE>92&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_CREDIT_NOTE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;=1200108906&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>92&lt;/SEQ_NO>  &lt;PATH>-117-292&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108876&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VAT INVOICE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108876&amp;amp;actionId=1200108876&amp;amp;mode=R&amp;amp;actionName=LEASE_SALE_INVOICE&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>98&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_SALE_INVOICE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200108876\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>98&lt;/SEQ_NO>  &lt;PATH>-117-298&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001024&lt;/ACTION_ID>  &lt;PARENT_ID/>  &lt;DISPLAY_NAME>MASTERS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>MASTER&lt;/MENU_TYPE>  &lt;SEQUENCE>18&lt;/SEQUENCE>  &lt;SCREEN_NAME>GMM&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>18&lt;/SEQ_NO>  &lt;PATH>-118&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001025&lt;/ACTION_ID>  &lt;PARENT_ID>1000001024&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>MASTER&lt;/MENU_TYPE>  &lt;SEQUENCE>105&lt;/SEQUENCE>  &lt;SCREEN_NAME>MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>105&lt;/SEQ_NO>  &lt;PATH>-118-2105&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106525&lt;/ACTION_ID>  &lt;PARENT_ID>1000001025&lt;/PARENT_ID>  &lt;DISPLAY_NAME>SALESMANAGER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>genericMasterAction.do?actionPerformed=displayData&amp;amp;screenId=1100106525&amp;amp;actionId=1100106525&amp;amp;mode=M&amp;amp;actionName=QM_SALESMANAGER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_SALESMANAGER&lt;/MENU_TYPE>  &lt;SEQUENCE>126&lt;/SEQUENCE>  &lt;SCREEN_NAME>SALESMANAGER (MAKER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>genericMasterAction.do?actionPerformed=displaydata&amp;amp;masterId=1100106525&amp;amp;mode=M&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>126&lt;/SEQ_NO>  &lt;PATH>-118-2105-3126&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001026&lt;/ACTION_ID>  &lt;PARENT_ID>1000001024&lt;/PARENT_ID>  &lt;DISPLAY_NAME>AUTHOR&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>MASTER&lt;/MENU_TYPE>  &lt;SEQUENCE>305&lt;/SEQUENCE>  &lt;SCREEN_NAME>AUTHOR&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>305&lt;/SEQ_NO>  &lt;PATH>-118-2305&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001300&lt;/ACTION_ID>  &lt;PARENT_ID/>  &lt;DISPLAY_NAME>BATCH UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>313&lt;/SEQUENCE>  &lt;SCREEN_NAME>BATCH_UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>313&lt;/SEQ_NO>  &lt;PATH>-1313&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001301&lt;/ACTION_ID>  &lt;PARENT_ID>1000001300&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>314&lt;/SEQUENCE>  &lt;SCREEN_NAME>BATCH UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>314&lt;/SEQ_NO>  &lt;PATH>-1313-2314&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>9200109154&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>AP INT GL MAP CONFIG&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=9200109154&amp;amp;actionId=9200109154&amp;amp;mode=M&amp;amp;moduleId=AP_INT_GL_MAP_CONFIG&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>AP INT GL MAP CONFIG&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106475&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BULK RECEIPT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106475&amp;amp;actionId=1100106475&amp;amp;mode=M&amp;amp;moduleId=OTC_IMD&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>BULK RECEIPT&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108861&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>INSTRUMENT MANAGEMENT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108861&amp;amp;actionId=1200108861&amp;amp;mode=M&amp;amp;moduleId=1200000300&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>INSTR MANAGEMENT MAKER/AUTHOR&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109167&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE REGISTRATION TRACKING&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109167&amp;amp;actionId=1200109167&amp;amp;mode=M&amp;amp;moduleId=LEASE_REG_DTL&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE REGISTRATION TRACKING&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106491&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>NEGATIVE LIST&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>NEGATIVE LIST UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109156&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>OFFLINE INSURANCE DETAILS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109156&amp;amp;actionId=1200109156&amp;amp;mode=M&amp;amp;moduleId=OFFLINE_INS_DTLS&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>OFFLINE INSURANCE DETAILS&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106494&lt;/ACTION_ID>  &lt;PARENT_ID>1100106491&lt;/PARENT_ID>  &lt;DISPLAY_NAME>SDN&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106494&amp;amp;actionId=1100106494&amp;amp;mode=M&amp;amp;moduleId=SDN&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>SDN NEGATIVE LIST UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315-41&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106496&lt;/ACTION_ID>  &lt;PARENT_ID>1100106491&lt;/PARENT_ID>  &lt;DISPLAY_NAME>TALIBAN&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106496&amp;amp;actionId=1100106496&amp;amp;mode=M&amp;amp;moduleId=TALIBAN&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>2&lt;/SEQUENCE>  &lt;SCREEN_NAME>TALIBAN NEGATIVE LIST UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>2&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315-42&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106493&lt;/ACTION_ID>  &lt;PARENT_ID>1100106491&lt;/PARENT_ID>  &lt;DISPLAY_NAME>AL-QAIDA&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106493&amp;amp;actionId=1100106493&amp;amp;mode=M&amp;amp;moduleId=ALQAIDA&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>AL-QAIDA NEGATIVE LIST UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315-43&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109123&lt;/ACTION_ID>  &lt;PARENT_ID>1100106491&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MCIB&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109123&amp;amp;actionId=1200109123&amp;amp;mode=M&amp;amp;moduleId=MCIB&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>MCIB&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315-43&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109105&lt;/ACTION_ID>  &lt;PARENT_ID>1100106491&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CAUTION LIST&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109105&amp;amp;actionId=1200109105&amp;amp;mode=M&amp;amp;moduleId=CAUTION_LIST&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>CAUTION LIST&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315-4315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100010120&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PDC/ECS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100010120&amp;amp;actionId=1100010120&amp;amp;mode=M&amp;amp;moduleId=PDC&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PDC/ECS&lt;/MENU_TYPE>  &lt;SEQUENCE>316&lt;/SEQUENCE>  &lt;SCREEN_NAME>PDC/ECS&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>316&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3316&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106572&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>QT FLEET DETAIL UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106572&amp;amp;actionId=1100106572&amp;amp;mode=M&amp;amp;moduleId=BH00000195&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QT_FLEET_DETAIL&lt;/MENU_TYPE>  &lt;SEQUENCE>320&lt;/SEQUENCE>  &lt;SCREEN_NAME>QT_FLEET_DETAIL UPLOAD  (AUTOAUTH)&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>320&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3320&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106753&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE ASSET PRICE MST UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106753&amp;amp;actionId=1200106753&amp;amp;mode=M&amp;amp;moduleId=1200000199&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_ASSET_PRICE_MST&lt;/MENU_TYPE>  &lt;SEQUENCE>321&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_ASSET_PRICE_MST UPLOAD  (MAKER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>321&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3321&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106841&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE ASSET VENDOR UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106841&amp;amp;actionId=1200106841&amp;amp;mode=M&amp;amp;moduleId=1200000209&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_ASSET_VENDOR&lt;/MENU_TYPE>  &lt;SEQUENCE>321&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_ASSET_VENDOR  (MAKER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>321&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3321&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106844&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE VENDOR GSTIN UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106844&amp;amp;actionId=1200106844&amp;amp;mode=M&amp;amp;moduleId=1200000210&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_VENDOR_GSTIN&lt;/MENU_TYPE>  &lt;SEQUENCE>321&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_VENDOR_GSTIN  (MAKER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>321&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3321&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106730&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MANUAL VOUCHER UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106730&amp;amp;actionId=1100106730&amp;amp;mode=M&amp;amp;moduleId=BH00000196&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>321&lt;/SEQUENCE>  &lt;SCREEN_NAME>MANUAL VOUCHER UPLOAD (AUTOAUTH)&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>321&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3321&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108938&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER INVOICE CONFIG UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108938&amp;amp;actionId=1200108938&amp;amp;mode=M&amp;amp;moduleId=1200000213&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_CUST_INVOICE_CONFIG_DTLS&lt;/MENU_TYPE>  &lt;SEQUENCE>322&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER INVOICE CONFIG UPLOAD (MAKER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>322&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3322&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106834&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE BATTERY PRICE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106834&amp;amp;actionId=1200106834&amp;amp;mode=M&amp;amp;moduleId=1200000207&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_BATTERY_PRICE&lt;/MENU_TYPE>  &lt;SEQUENCE>322&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_BATTERY_PRICE UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>322&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3322&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106763&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE TYRE PRICE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106763&amp;amp;actionId=1200106763&amp;amp;mode=M&amp;amp;moduleId=1200000202&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_TYRE_PRICE&lt;/MENU_TYPE>  &lt;SEQUENCE>322&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_TYRE_PRICE UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>322&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3322&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108934&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MANUAL ADVICE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108934&amp;amp;actionId=1200108934&amp;amp;mode=M&amp;amp;moduleId=1200000212&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QT_MANUALADVICE&lt;/MENU_TYPE>  &lt;SEQUENCE>322&lt;/SEQUENCE>  &lt;SCREEN_NAME>MANUAL ADVICE UPLOAD (MAKER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>322&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3322&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108997&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE ASSET CESSMAPPING UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108997&amp;amp;actionId=1200108997&amp;amp;mode=M&amp;amp;moduleId=1200000303&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_ASSET_CESSMAPPING&lt;/MENU_TYPE>  &lt;SEQUENCE>323&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE ASSET CESSMAPPING UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>323&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3323&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106771&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE ASSET VARIANT UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106771&amp;amp;actionId=1200106771&amp;amp;mode=M&amp;amp;moduleId=1200000205&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_ASSET_VARIANT&lt;/MENU_TYPE>  &lt;SEQUENCE>323&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_ASSET_VARIANT UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>323&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3323&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108948&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE FC AMNT FORMULA UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108948&amp;amp;actionId=1200108948&amp;amp;mode=M&amp;amp;moduleId=1200000301&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_FC_AMNT_FORMULA&lt;/MENU_TYPE>  &lt;SEQUENCE>323&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_FC_AMNT_FORMULA UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>323&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3323&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108965&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE SERVICE CHRG AMT UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108965&amp;amp;actionId=1200108965&amp;amp;mode=M&amp;amp;moduleId=1200000302&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_SERVICE_CHRG_AMT&lt;/MENU_TYPE>  &lt;SEQUENCE>323&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE SERVICE CHRG AMT UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>323&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3323&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109004&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE VENDOR BANK UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109004&amp;amp;actionId=1200109004&amp;amp;mode=M&amp;amp;moduleId=1200000304&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_VENDOR_BANK_MASTER&lt;/MENU_TYPE>  &lt;SEQUENCE>323&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE VENDOR BANK UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>323&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3323&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109006&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE VENDOR MAPPING UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109006&amp;amp;actionId=1200109006&amp;amp;mode=M&amp;amp;moduleId=1200000305&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_VENDOR_MAPPING&lt;/MENU_TYPE>  &lt;SEQUENCE>323&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE VENDOR MAPPING UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>323&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3323&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106831&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE MAINT SLABS UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106831&amp;amp;actionId=1200106831&amp;amp;mode=M&amp;amp;moduleId=1200000206&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_MAINT_SLABS&lt;/MENU_TYPE>  &lt;SEQUENCE>324&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_MAINT_SLABS UPLOAD &lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>324&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3324&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109034&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BULK CLOSURE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109034&amp;amp;actionId=1200109034&amp;amp;mode=M&amp;amp;moduleId=BULK_CLOSURE&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>350&lt;/SEQUENCE>  &lt;SCREEN_NAME>BULK CLOSURE MAKER/AUTHOR&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>350&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3350&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106724&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CHARGE X GST UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106724&amp;amp;actionId=1100106724&amp;amp;mode=M&amp;amp;moduleId=BH00000023&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_CHARGE_X_GST&lt;/MENU_TYPE>  &lt;SEQUENCE>353&lt;/SEQUENCE>  &lt;SCREEN_NAME>CHARGE_X_GST UPLOAD  (MAKER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>353&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3353&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106838&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE RESIDUAL VALUE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106838&amp;amp;actionId=1200106838&amp;amp;mode=M&amp;amp;moduleId=1200000208&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_RESIDUAL_VALUE&lt;/MENU_TYPE>  &lt;SEQUENCE>996&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_RESIDUAL_VALUE UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>996&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3996&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001302&lt;/ACTION_ID>  &lt;PARENT_ID>1000001300&lt;/PARENT_ID>  &lt;DISPLAY_NAME>AUTHOR&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>320&lt;/SEQUENCE>  &lt;SCREEN_NAME>BATCH UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>320&lt;/SEQ_NO>  &lt;PATH>-1313-2320&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>9200109155&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>AP INT GL MAP CONFIG&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=9200109155&amp;amp;actionId=9200109155&amp;amp;mode=A&amp;amp;moduleId=AP_INT_GL_MAP_CONFIG&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>AP INT GL MAP CONFIG&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109036&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BULK CLOSURE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109036&amp;amp;actionId=1200109036&amp;amp;mode=A&amp;amp;moduleId=BULK_CLOSURE&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>BULK CLOSURE MAKER/AUTHOR&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109045&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BULK RECEIPT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109045&amp;amp;actionId=1200109045&amp;amp;mode=A&amp;amp;moduleId=OTC_IMD&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>BULK RECEIPT&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108863&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER RECEIPT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108863&amp;amp;actionId=1200108863&amp;amp;mode=A&amp;amp;moduleId=CUSTOMER_RECEIPT&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER RECEIPT MAKER/AUTHOR&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106754&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE ASSET PRICE MST UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106754&amp;amp;actionId=1200106754&amp;amp;mode=A&amp;amp;moduleId=1200000199&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_ASSET_PRICE_MST&lt;/MENU_TYPE>  &lt;SEQUENCE>322&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_ASSET_PRICE_MST UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>322&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3322&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100010121&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PDC/ECS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100010121&amp;amp;actionId=1100010121&amp;amp;mode=A&amp;amp;moduleId=PDC&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PDC/ECS&lt;/MENU_TYPE>  &lt;SEQUENCE>322&lt;/SEQUENCE>  &lt;SCREEN_NAME>PDC/ECS&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>322&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3322&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>9200109157&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE VENDOR BANK UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=9200109157&amp;amp;actionId=9200109157&amp;amp;mode=A&amp;amp;moduleId=1200000304&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_VENDOR_BANK_MASTER&lt;/MENU_TYPE>  &lt;SEQUENCE>323&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE VENDOR BANK UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>323&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3323&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108935&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MANUAL ADVICE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108935&amp;amp;actionId=1200108935&amp;amp;mode=A&amp;amp;moduleId=1200000212&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QT_MANUALADVICE&lt;/MENU_TYPE>  &lt;SEQUENCE>323&lt;/SEQUENCE>  &lt;SCREEN_NAME>MANUAL ADVICE UPLOAD (AUTHOR)&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>323&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3323&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106772&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE ASSET VARIANT UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106772&amp;amp;actionId=1200106772&amp;amp;mode=A&amp;amp;moduleId=1200000205&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_ASSET_VARIANT&lt;/MENU_TYPE>  &lt;SEQUENCE>324&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_ASSET_VARIANT UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>324&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3324&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108949&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE FC AMNT FORMULA UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108949&amp;amp;actionId=1200108949&amp;amp;mode=A&amp;amp;moduleId=1200000301&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_FC_AMNT_FORMULA&lt;/MENU_TYPE>  &lt;SEQUENCE>324&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_FC_AMNT_FORMULA UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>324&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3324&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108966&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE SERVICE CHRG AMT UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108966&amp;amp;actionId=1200108966&amp;amp;mode=A&amp;amp;moduleId=1200000302&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_SERVICE_CHRG_AMT&lt;/MENU_TYPE>  &lt;SEQUENCE>324&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE SERVICE CHRG AMT UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>324&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3324&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106842&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE ASSET VENDOR UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106842&amp;amp;actionId=1200106842&amp;amp;mode=A&amp;amp;moduleId=1200000209&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_ASSET_VENDOR&lt;/MENU_TYPE>  &lt;SEQUENCE>326&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_ASSET_VENDOR  (AUTHOR)&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>326&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3326&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106845&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE VENDOR GSTIN UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106845&amp;amp;actionId=1200106845&amp;amp;mode=A&amp;amp;moduleId=1200000210&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_VENDOR_GSTIN&lt;/MENU_TYPE>  &lt;SEQUENCE>326&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_VENDOR_GSTIN  (AUTHOR)&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>326&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3326&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106732&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MANUAL VOUCHER UPLOAD AUTHOR&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106732&amp;amp;actionId=1100106732&amp;amp;mode=A&amp;amp;moduleId=BH00000196&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>326&lt;/SEQUENCE>  &lt;SCREEN_NAME>MANUAL VOUCHER UPLOAD AUTHOR&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>326&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3326&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106835&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE BATTERY PRICE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106835&amp;amp;actionId=1200106835&amp;amp;mode=A&amp;amp;moduleId=1200000207&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_BATTERY_PRICE&lt;/MENU_TYPE>  &lt;SEQUENCE>327&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_BATTERY_PRICE UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>327&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3327&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106764&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE TYRE PRICE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106764&amp;amp;actionId=1200106764&amp;amp;mode=A&amp;amp;moduleId=1200000202&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_TYRE_PRICE&lt;/MENU_TYPE>  &lt;SEQUENCE>327&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_TYRE_PRICE UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>327&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3327&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106832&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE MAINT SLABS UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>BATCHUPLOADACTION.DO?ACTIONPERFORMED=BATCHUPLOAD&amp;amp;SCREENID=1200106832&amp;amp;ACTIONID=1200106832&amp;amp;MODE=A&amp;amp;MODULEID=1200000206&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_MAINT_SLABS&lt;/MENU_TYPE>  &lt;SEQUENCE>329&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_MAINT_SLABS UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>329&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3329&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109104&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>NEGATIVE LIST&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>332&lt;/SEQUENCE>  &lt;SCREEN_NAME>NEGATIVE LIST VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>332&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3332&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109106&lt;/ACTION_ID>  &lt;PARENT_ID>1200109104&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CAUTION LIST&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109106&amp;amp;actionId=1200109106&amp;amp;mode=A&amp;amp;moduleId=CAUTION_LIST&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>CAUTION LIST&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3332-4315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106725&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CHARGE X GST UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106725&amp;amp;actionId=1100106725&amp;amp;mode=A&amp;amp;moduleId=BH00000023&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_CHARGE_X_GST&lt;/MENU_TYPE>  &lt;SEQUENCE>351&lt;/SEQUENCE>  &lt;SCREEN_NAME>CHARGE_X_GST UPLOAD  (AUTHOR)&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>351&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3351&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106839&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE RESIDUAL VALUE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106839&amp;amp;actionId=1200106839&amp;amp;mode=A&amp;amp;moduleId=1200000208&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_RESIDUAL_VALUE&lt;/MENU_TYPE>  &lt;SEQUENCE>997&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_RESIDUAL_VALUE UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>997&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3997&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001303&lt;/ACTION_ID>  &lt;PARENT_ID>1000001300&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>330&lt;/SEQUENCE>  &lt;SCREEN_NAME>BATCH UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>330&lt;/SEQ_NO>  &lt;PATH>-1313-2330&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>9200109156&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>AP INT GL MAP CONFIG&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=9200109156&amp;amp;actionId=9200109156&amp;amp;mode=V&amp;amp;moduleId=AP_INT_GL_MAP_CONFIG&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>AP INT GL MAP CONFIG&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109168&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE REGISTRATION TRACKING&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109168&amp;amp;actionId=1200109168&amp;amp;mode=V&amp;amp;moduleId=LEASE_REG_DTL&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>316&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE REGISTRATION TRACKING&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>316&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3316&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109157&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>OFFLINE INSURANCE DETAILS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109157&amp;amp;actionId=1200109157&amp;amp;mode=V&amp;amp;moduleId=OFFLINE_INS_DTLS&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>316&lt;/SEQUENCE>  &lt;SCREEN_NAME>OFFLINE INSURANCE DETAILS&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>316&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3316&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106755&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE ASSET PRICE MST UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106755&amp;amp;actionId=1200106755&amp;amp;mode=V&amp;amp;moduleId=1200000199&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_ASSET_PRICE_MST&lt;/MENU_TYPE>  &lt;SEQUENCE>321&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_ASSET_PRICE_MST UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>321&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3321&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106731&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MANUAL VOUCHER UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106731&amp;amp;actionId=1100106731&amp;amp;mode=V&amp;amp;moduleId=BH00000196&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>322&lt;/SEQUENCE>  &lt;SCREEN_NAME>MANUAL VOUCHER UPLOAD (VIEWER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>322&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3322&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108940&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER INVOICE CONFIG UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108940&amp;amp;actionId=1200108940&amp;amp;mode=V&amp;amp;moduleId=1200000213&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_CUST_INVOICE_CONFIG_DTLS&lt;/MENU_TYPE>  &lt;SEQUENCE>324&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER INVOICE CONFIG UPLOAD (VIEWER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>324&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3324&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108936&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MANUAL ADVICE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108936&amp;amp;actionId=1200108936&amp;amp;mode=V&amp;amp;moduleId=1200000212&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QT_MANUALADVICE&lt;/MENU_TYPE>  &lt;SEQUENCE>324&lt;/SEQUENCE>  &lt;SCREEN_NAME>MANUAL ADVICE UPLOAD (VIEWER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>324&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3324&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106773&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE ASSET VARIANT UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106773&amp;amp;actionId=1200106773&amp;amp;mode=V&amp;amp;moduleId=1200000205&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_ASSET_VARIANT&lt;/MENU_TYPE>  &lt;SEQUENCE>325&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_ASSET_VARIANT UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>325&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3325&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108950&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE FC AMNT FORMULA UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108950&amp;amp;actionId=1200108950&amp;amp;mode=V&amp;amp;moduleId=1200000301&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_FC_AMNT_FORMULA&lt;/MENU_TYPE>  &lt;SEQUENCE>325&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_FC_AMNT_FORMULA UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>325&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3325&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108967&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE SERVICE CHRG AMT UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108967&amp;amp;actionId=1200108967&amp;amp;mode=V&amp;amp;moduleId=1200000302&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_SERVICE_CHRG_AMT&lt;/MENU_TYPE>  &lt;SEQUENCE>325&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE SERVICE CHRG AMT UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>325&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3325&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109005&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE VENDOR BANK UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109005&amp;amp;actionId=1200109005&amp;amp;mode=V&amp;amp;moduleId=1200000304&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_VENDOR_BANK_MASTER&lt;/MENU_TYPE>  &lt;SEQUENCE>325&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE VENDOR BANK UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>325&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3325&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109007&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE VENDOR MAPPING UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109007&amp;amp;actionId=1200109007&amp;amp;mode=V&amp;amp;moduleId=1200000305&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_VENDOR_MAPPING&lt;/MENU_TYPE>  &lt;SEQUENCE>325&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE VENDOR MAPPING UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>325&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3325&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106476&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BULK RECEIPT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106476&amp;amp;actionId=1100106476&amp;amp;mode=V&amp;amp;moduleId=OTC_IMD&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>332&lt;/SEQUENCE>  &lt;SCREEN_NAME>BULK RECEIPT&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>332&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3332&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108866&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER RECEIPT VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108866&amp;amp;actionId=1200108866&amp;amp;mode=V&amp;amp;moduleId=CUSTOMER_RECEIPT&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>332&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER RECEIPT VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>332&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3332&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108862&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>INSTRUMENT MANAGEMENT VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108861&amp;amp;actionId=1200108861&amp;amp;mode=V&amp;amp;moduleId=1200000300&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>332&lt;/SEQUENCE>  &lt;SCREEN_NAME>INSTR MANAGEMENT VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>332&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3332&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106490&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>NEGATIVE LIST&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>332&lt;/SEQUENCE>  &lt;SCREEN_NAME>NEGATIVE LIST VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>332&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3332&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106484&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>RESCH VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106484&amp;amp;actionId=1100106484&amp;amp;mode=V&amp;amp;moduleId=RESCH&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>332&lt;/SEQUENCE>  &lt;SCREEN_NAME>RESCH&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>332&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3332&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106495&lt;/ACTION_ID>  &lt;PARENT_ID>1100106490&lt;/PARENT_ID>  &lt;DISPLAY_NAME>SDN&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106495&amp;amp;actionId=1100106495&amp;amp;mode=V&amp;amp;moduleId=SDN&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>SDN NEGATIVE LIST&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3332-41&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106497&lt;/ACTION_ID>  &lt;PARENT_ID>1100106490&lt;/PARENT_ID>  &lt;DISPLAY_NAME>TALIBAN&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106497&amp;amp;actionId=1100106497&amp;amp;mode=V&amp;amp;moduleId=TALIBAN&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>2&lt;/SEQUENCE>  &lt;SCREEN_NAME>TALIBAN NEGATIVE LIST&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>2&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3332-42&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106492&lt;/ACTION_ID>  &lt;PARENT_ID>1100106490&lt;/PARENT_ID>  &lt;DISPLAY_NAME>AL-QAIDA&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106492&amp;amp;actionId=1100106492&amp;amp;mode=V&amp;amp;moduleId=ALQAIDA&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>AL-QAIDA NEGATIVE LIST&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3332-43&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109122&lt;/ACTION_ID>  &lt;PARENT_ID>1100106490&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MCIB&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109122&amp;amp;actionId=1200109122&amp;amp;mode=V&amp;amp;moduleId=MCIB&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>MCIB&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3332-43&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109107&lt;/ACTION_ID>  &lt;PARENT_ID>1100106490&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CAUTION LIST&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109107&amp;amp;actionId=1200109107&amp;amp;mode=V&amp;amp;moduleId=CAUTION_LIST&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>CAUTION LIST&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3332-4315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109035&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BULK CLOSURE VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109035&amp;amp;actionId=1200109035&amp;amp;mode=V&amp;amp;moduleId=BULK_CLOSURE&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>333&lt;/SEQUENCE>  &lt;SCREEN_NAME>BULK CLOSURE&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>333&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3333&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100010128&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PDC/ECS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100010128&amp;amp;actionId=1100010128&amp;amp;mode=V&amp;amp;moduleId=PDC&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PDC/ECS&lt;/MENU_TYPE>  &lt;SEQUENCE>335&lt;/SEQUENCE>  &lt;SCREEN_NAME>PDC/ECS &lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>335&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3335&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106843&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE ASSET VENDOR UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106843&amp;amp;actionId=1200106843&amp;amp;mode=V&amp;amp;moduleId=1200000209&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_ASSET_VENDOR&lt;/MENU_TYPE>  &lt;SEQUENCE>338&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_ASSET_VENDOR  (VIEWER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>338&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3338&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106846&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE VENDOR GSTIN UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106846&amp;amp;actionId=1200106846&amp;amp;mode=V&amp;amp;moduleId=1200000210&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_VENDOR_GSTIN&lt;/MENU_TYPE>  &lt;SEQUENCE>338&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_VENDOR_GSTIN  (VIEWER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>338&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3338&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106836&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE BATTERY PRICE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106836&amp;amp;actionId=1200106836&amp;amp;mode=V&amp;amp;moduleId=1200000207&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_BATTERY_PRICE&lt;/MENU_TYPE>  &lt;SEQUENCE>339&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_BATTERY_PRICE UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>339&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3339&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106765&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE TYRE PRICE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106765&amp;amp;actionId=1200106765&amp;amp;mode=V&amp;amp;moduleId=1200000202&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_TYRE_PRICE&lt;/MENU_TYPE>  &lt;SEQUENCE>339&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_TYRE_PRICE UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>339&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3339&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106833&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE MAINT SLABS UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>BATCHUPLOADACTION.DO?ACTIONPERFORMED=BATCHUPLOAD&amp;amp;SCREENID=1200106833&amp;amp;ACTIONID=1200106833&amp;amp;MODE=V&amp;amp;MODULEID=1200000206&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_MAINT_SLABS&lt;/MENU_TYPE>  &lt;SEQUENCE>341&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_MAINT_SLABS UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>341&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3341&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106726&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CHARGE X GST UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106726&amp;amp;actionId=1100106726&amp;amp;mode=V&amp;amp;moduleId=BH00000023&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_CHARGE_X_GST&lt;/MENU_TYPE>  &lt;SEQUENCE>352&lt;/SEQUENCE>  &lt;SCREEN_NAME>CHARGE_X_GST UPLOAD  (VIEWER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>352&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3352&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106840&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE RESIDUAL VALUE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106840&amp;amp;actionId=1200106840&amp;amp;mode=V&amp;amp;moduleId=1200000208&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_RESIDUAL_VALUE&lt;/MENU_TYPE>  &lt;SEQUENCE>998&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_RESIDUAL_VALUE UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>998&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3998&lt;/PATH> &lt;/ROW>&lt;/ROWSET>&quot;;
		           // alert(menuDataXml)
					setMenuData(menuDataXml);
					renderItemsInto(&quot;verticalMenu&quot;);
		
		
					
				
			
		
	

					

	
		

		
		
			




  
 
    
  miFIN
    
	
	
	    
	
	
	

  
  
  
  
	
  
   	  
			 		 
						  
						  
						  
					  
		
 
 




		
	
	      	
   	
   		
		
		
		
		
			
							CHARGE DETAILS
							
								
									 
									
									
									
			
		
		
				
				
		 
         
         
                  
         
         
         
          
          
          
          
          
          
          
             
        
         
         
		
				
				
								
				
		
		
			
		      	
					
						
							
					 
						 Charge Type
						 Dr/Cr Note
						 Misc. Bill
						
						 Invoice No
						 Charge Id
						 Charge Amount
						 Charge/Invoice Date
						Bill Start Date
						Bill End Date
						 Payment Due Date
						
						 Maker Remarks  
				        
				        
							
				        
				        
						
					    	 Send To Author  
						
						
					
				
			
			
				
				SelectReceivablePayable SelectPRO RATA INTEREST        
			
		
		         
	     
	     
	     
	     
	
	              




	
		
		Qualtech Consultants Pvt. Ltd.
		
		
		VERSION 1.0.1.295
		
			
			   
				 
		
	
		
 


	var objText = null;			
	                  
	function refreshCustomerServiceInLos()
	{
	  var pageURL = $(location).attr(&quot;href&quot;);
       var pathName = pageURL.substring(pageURL.lastIndexOf('/') + 1,Number(pageURL.length));
       $(location).attr('href', pathName);
	 } 

$(window).on('load', function(){
	if(!$(&quot;div&quot;).hasClass(&quot;menu_tab&quot;)){
	
		$(&quot;body:not(.body_searchlist)&quot;).addClass(&quot;menuHavingBody&quot;);
		//$(&quot;body&quot;).not(&quot;.body_searchlist&quot;).addClass(&quot;menuHavingBody&quot;);
		//$(&quot;body&quot;).addClass(&quot;menuHavingBody&quot;);
	}
	if($(&quot;.subheaderSection div div:nth-of-type(2)&quot;).text()!==&quot;&quot;){
	$(&quot;.slimScrollDiv&quot;).addClass(&quot;responsiveMenu&quot;);
	}	
	$(&quot;#securityCheckList&quot;).next(&quot;table&quot;).find(&quot;td:nth-of-type(1)&quot;).css(&quot;width&quot;,&quot;5%&quot;);
	$(&quot;#secContainer #securityCheckList tr:not(':nth-child(1)') td:nth-child(9)&quot;).css(&quot;text-align&quot;,&quot;right&quot;);
	if($(document).height()>($(window).height())){
	$(&quot;.text_footer&quot;).addClass(&quot;posrell&quot;);
	}
	$(&quot;a,.blueBotton&quot;).click(function(){
	if($(document).height()>($(window).height())){$(&quot;.text_footer&quot;).addClass(&quot;posrell&quot;);}
	});

});


function disableAllElementsAjax()
		    {
		    	
		  	   
	                  for(count=0; count &lt; document.forms[0].elements.length; count+=1)
				        {
				        	theelement = document.forms[0].elements[count];
				            if(theelement.name != null &amp;&amp; theelement.name != &quot;btn_one&quot; &amp;&amp;theelement.name !=&quot;CreditOfficerHistory&quot; &amp;&amp;theelement.name != &quot;btnCam&quot; &amp;&amp;theelement.name != &quot;btnSanction&quot;)
				            {
				            	theelement.disabled = true;
				            }
				        }
				      
			  
		    }           

$(window).on('load', function(){
	if($(&quot;.subheaderSection div div:nth-of-type(2)&quot;).text()!==&quot;&quot;){
		$(&quot;.slimScrollDiv&quot;).addClass(&quot;responsiveMenu&quot;);
	}	
	$(&quot;.menu_tab&quot;).parents(&quot;body&quot;).addClass(&quot;menuHavingBody&quot;);
	//1.0.0.1 start
	if(!$(&quot;textarea&quot;).prop(&quot;disabled&quot;))
	{
			$(&quot;textarea&quot;).wrap('&lt;span class=&quot;textarea-span&quot; style=&quot;position: relative;float: left;width: 100%;&quot;>&lt;/span>');
			$(&quot;.textarea-span&quot;).append('&lt;span class=&quot;edit-textarea&quot;>&lt;/span>');
	
	} // 1.0.0.1 end	
	$(document).on(&quot;click&quot;,&quot;.edit-textarea&quot;, function(){
			objText = $(this);
			//Start Added by 1.0.0.2
			if(objText.prev(&quot;textarea&quot;).prop(&quot;disabled&quot;))
				return;
			//End Added by 1.0.0.2
			$(&quot;.justbeforeform&quot;).remove();
			$(&quot;.textarea-popup-content&quot;).remove();
			var eachtareacontent = $(this).prev(&quot;textarea&quot;).val();
			$(&quot;body&quot;).append('&lt;div class=&quot;justbeforeform&quot;>&lt;/div> &lt;div class=&quot;textarea-popup-content&quot;>&lt;/div>');
			$(&quot;.textarea-popup-content&quot;).wrapInner('&lt;div class=&quot;div_popup&quot; contenteditable=&quot;true&quot;>&lt;/div>');
			$(&quot;.textarea-popup-content&quot;).animate({top: &quot;100px&quot;});
			
			$(&quot;.div_popup&quot;).text(eachtareacontent);
			$(&quot;.div_popup&quot;).after('&lt;input type=&quot;button&quot; class=&quot;ok_btn&quot; value=&quot;Close&quot; style=&quot;float:right&quot; />');
			$('html, body').animate({scrollTop : 0},800);
		
			$(document).on(&quot;click&quot;,&quot;.ok_btn&quot;,function(){
			var tyu=$(&quot;.div_popup&quot;).text();
		
				objText.prev(&quot;textarea&quot;).val(tyu);
				$(&quot;.ok_btn&quot;).parent(&quot;.textarea-popup-content&quot;).siblings(&quot;.justbeforeform&quot;).fadeOut(0);
				$(&quot;.ok_btn&quot;).parent(&quot;.textarea-popup-content&quot;).animate({top: &quot;-600px&quot;});
				$(&quot;.ok_btn&quot;).parent(&quot;.textarea-popup-content&quot;).siblings(&quot;.justbeforeform&quot;).css(&quot;display&quot;,&quot;none&quot;);
				//$(&quot;.ok_btn&quot;).parent(&quot;.textarea-popup-content&quot;).css(&quot;display&quot;,&quot;none&quot;);
			});
		});
});

       

	



  

/html[1]/body[@class=&quot;menuHavingBody&quot;]</value>
      <webElementGuid>919aba8b-05ac-44ef-81e2-729b8f3456a4</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[@class=&quot;menuHavingBody&quot;]</value>
      <webElementGuid>698ba6f7-efd3-4f47-8776-0c76fcb84948</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//body</value>
      <webElementGuid>adc88b13-e9f4-4b4f-85d3-f3b5c4081bce</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//body[(text() = concat(&quot;
	
			


 
 
 
	
	
	
	
	
	
	 







//Code added by Naveen Baghel::10/12/2015


setTimeout(&quot;setToolTip();&quot;,100);

function setTitleforVarFields()
{
	for(count=0; count &lt; document.forms[0].elements.length; count+=1)
    {
    	theelement = document.forms[0].elements[count];
    	if(theelement)
        {
	        if(theelement.type == &quot;text&quot;)
	        {
	        	theelement.title = (theelement.value).toUpperCase();
	        	
	        }
	      
	        
	     }
    }   
}


   function setToolTip()

   {try{

   if(document.forms[0])

   {

    for(count=0; count &lt; document.forms[0].elements.length; count+=1)

        {

          theelement = document.forms[0].elements[count];

                   if(theelement.options != null)

                   {
                     showTooltip(theelement);
                   }

                   else

                   {

                    if(theelement.type!=&quot;checkbox&quot; &amp;&amp; theelement.type!=&quot;radio&quot; &amp;&amp;  theelement.value!=null &amp;&amp; theelement.value!=&quot;&quot; )

                     theelement.title=theelement.value;  
                     changeToolTipText(theelement);     
                   }

        }
    }

    }catch(err){

    //alert(err);

    }

    }
    
    function setToolTipText(theelement)
    {
       theelement.title=theelement.value;   
    }

function showTooltip(theelement)

                        {    
                                                                                             

            for(i=0; i&lt;theelement.options.length; i++)

               {

                 theelement.options[i].title=theelement.options[i].text;

               }

               if(theelement.addEventListener){

                                                  theelement.addEventListener(&quot;change&quot;, function(e){

                                                    e = e || event;

                                                    changeToolTip();

                                                  }, false);
                                                 

                                                }

                                                else if(theelement.attachEvent){

                                                  theelement.attachEvent(&quot;onchange&quot;, function(e){

                                                    e = e || event;

                                                    changeToolTip();

                                                  });
                                                  
                                                }

                                                try

                                                {

                if(parseInt(theelement.options.length)!=0)

               theelement.title=theelement.options[theelement.selectedIndex].text;

               else

               theelement.title=theelement.value;

               }

               catch(err)

               {

                        theelement.title=&quot;&quot;;

               }

                        }

        
   function changeToolTipText(theelement)
   {
       if(theelement.addEventListener){

                                                  theelement.addEventListener(&quot;keyup&quot;, function(e){

                                                    e = e || event;

                                                    setToolTipText(theelement);

                                                  }, false);
                                                 

                                                }

                                                else if(theelement.attachEvent){

                                                  theelement.attachEvent(&quot;onkeyup&quot;, function(e){

                                                    e = e || event;

                                                    setToolTipText(theelement);

                                                  });
                                                  
                                                }
   }
        

    function changeToolTip()

    {try{

     for(count=0; count &lt; document.forms[0].elements.length; count+=1)

       {

         theelement = document.forms[0].elements[count];

           if(theelement.options != null)

             {

                if(parseInt(theelement.options.length)!=0)

                 theelement.title=theelement.options[theelement.selectedIndex].text;

                 else

                 theelement.title=theelement.value;

             }
        }}

        catch(err){
       

        }

    }                        

//-----------------------Ends Here-----------------

var blockResponsechk=false;
var chkWin;
var chkWinMsg=&quot;&quot;;
var chkWinValue=false;
function blockClick()
            {
                 
                if(chkWin &amp;&amp; !chkWin.closed)
			{
			chkWinValue=true;
			if(chkWinMsg!=&quot;&quot;)
			{
			alert(chkWinMsg);
			}
			else {
			alert((&quot;Denomination window is not close. Please close it!&quot;).toUpperCase());
			}
			chkWin.focus();
			return false;
			}
			else {
				chkWinValue=false;
			}
               if((!document.getElementById(&quot;SaveLink&quot;)) || (!document.getElementById(&quot;SaveExitLink&quot;)))
                 {
                  if(blockResponsechk==true)
                  {
                        alert((&quot;Activity in Progress.Please wait...&quot;).toUpperCase());
                        return false;
                       
                  }
                  else
                  {                   	
                        blockResponsechk=true; 
                       setTimeout(&quot;setTimerReClick()&quot;,10000); // after 10 sec it&quot; , &quot;'&quot; , &quot;s call
                        return true;
                  }
                }
                
               
            }
           

	function setTimerReClick()
	{		
		blockResponsechk=false;
	}

//Disable right mouse click Script
//By Maximus (maximus@nsimail.com) w/ mods by DynamicDrive
//For full source code, visit http://www.dynamicdrive.com
//added by sunny to restrict refresh using ctrl +R
$(document).ready(function () {
    $(document).on(&quot;keydown&quot;, function(e) {
        e = e || window.event;
        if (e.ctrlKey) {
            var c = e.which || e.keyCode;
            if (c == 82) {
            	alert(message);
                e.preventDefault();
                e.stopPropagation();
                return false;
            }
        }
    });
});
//add end bby sunny
 var message=&quot;FUNCTION DISABLED!&quot;;

///////////////////////////////////
function clickIE4(){
if (event.button==2){
alert(message);
return false;
}
}

function clickNS4(e){
if (document.layers||document.getElementById&amp;&amp;!document.all){
if (e.which==2||e.which==3){
alert(message);
return false;
}
}
}

if (document.layers){
document.captureEvents(Event.MOUSEDOWN);
document.onmousedown=clickNS4;

}
else if (document.all&amp;&amp;!document.getElementById){
document.onmousedown=clickIE4;
}

document.oncontextmenu=new Function(&quot;alert(message);return false&quot;);




//This function changed by Ravikant for browser compatibility
function showKeyCode(e)
{
var src=src=e.srcElement;
if(src==null || src==&quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot; || src==undefined )
src=e.target;
var tag = src.tagName ? src.tagName.toUpperCase() : &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;; 
var typ = (tag == &quot; , &quot;'&quot; , &quot;INPUT&quot; , &quot;'&quot; , &quot;) ? src.type.toUpperCase() : &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;; 
var isTextArea = (tag == &quot; , &quot;'&quot; , &quot;TEXTAREA&quot; , &quot;'&quot; , &quot;); 
var isTextField = ((tag == &quot; , &quot;'&quot; , &quot;INPUT&quot; , &quot;'&quot; , &quot;) &amp;&amp; (typ == &quot; , &quot;'&quot; , &quot;TEXT&quot; , &quot;'&quot; , &quot;)); 
var isPassField= ((tag == &quot; , &quot;'&quot; , &quot;INPUT&quot; , &quot;'&quot; , &quot;) &amp;&amp; (typ == &quot; , &quot;'&quot; , &quot;PASSWORD&quot; , &quot;'&quot; , &quot;)); 
var isText = isTextField || isTextArea || isPassField; 

var disabled = isText ? src.disabled : false; 
var readOnly = isText ? src.readOnly : false; 

var keycode =(window.event) ? event.keyCode : e.keyCode;

if(keycode == 116)
{
alert((&quot; , &quot;'&quot; , &quot;Function Disabled!&quot; , &quot;'&quot; , &quot;).toUpperCase());
event.keyCode = 0;
event.returnValue = false;
return false;
}
var browser=(navigator.userAgent).toLowerCase();
if(browser.indexOf(&quot;msie&quot;)>0){
if(!isText)
{
	if(keycode == 8)
	{
		alert((&quot; , &quot;'&quot; , &quot;Function Disabled!&quot; , &quot;'&quot; , &quot;).toUpperCase());
		event.keyCode = 0;
		event.returnValue = false;
		return false;
	}
}
}

if (disabled || readOnly) 
 { 

if(keycode == 8)
{

	alert((&quot; , &quot;'&quot; , &quot;Function Disabled!&quot; , &quot;'&quot; , &quot;).toUpperCase());
	event.keyCode = 0;
	event.returnValue = false;
	return false;
} 
}
}

/*Satrt Added For Bud Id:13340*/
var myWindow;
/*End Added For Bud Id:13340*/	
	function onBeforeUnloadAction()
 	{
		document.forms[0].action = &quot;userAuthAction.do?dispatchMethod=logout&quot;;
        document.forms[0].method = &quot;post&quot;;
        document.forms[0].submit();
   }
   
   window.onbeforeunload = function(e)
   {
   		if(window.event &amp;&amp; !e)
   		e=window.event;
   		
   		if((e.clientX&lt;0) ||(e.clientY&lt;0))
 		{
 			/*Satrt Added For Bud Id:13340*/
 			if (myWindow &amp;&amp; myWindow.closed) 
 			{
 		 		return onBeforeUnloadAction();
 			}
 			/*End Added For Bud Id:13340*/
 			
 			/*Satrt Comment For Bud Id:13340*/
 			
 			//return onBeforeUnloadAction();
 			
 			/*Satrt Comment For Bud Id:13340*/
 			
 		}
 	}; 
 	function debugAlert(alertMesg)
 	{
 	var debugAlert=&quot;N&quot;;
 	if(debugAlert==&quot;Y&quot;)
 	alert(alertMesg);
 	}
 	
 	function testing()
 	{
 	   // fore testing start
	
	 $.ajax({  
			type : &quot;post&quot;,   
			url : &quot;BroadcastServiceRequestServlet&quot;,   
			ontext: document.body,			
			success : function(response) {			
				  
			   }			
		}); 
	// for testing end
 	   
 	}

 
 


	
	
			
				
					  
						
				
					Hi NAVIND
					
					
				
				
				
					
				
				
					
				
				
				
				
				
					
					
						
							
							
							25-JUL-2023
							
						
					
						 
					
					
					
					
					NAVIND
					
				
					
				Logout
			
			
				
					
				
				
				
					
				
				
				
				
			
			
		
	
		
		 
		
  
	

	 

 
		$(window).on(&quot; , &quot;'&quot; , &quot;load&quot; , &quot;'&quot; , &quot;, function() {
			
			$(this).parents(&quot;tr&quot;).removeClass(&quot;cstmtr&quot;);
			$(document).on(&quot;click&quot;,&quot;#xyz table tr td a&quot;,function(){
			$(this).parents(&quot;tr&quot;).addClass(&quot;cstmtr&quot;);
			
			});
			$(document).on(&quot;blur&quot;,&quot;#xyz table tr td a&quot;,function(){
			$(this).parents(&quot;tr&quot;).removeClass(&quot;cstmtr&quot;);
			
			});
			$(&quot;#llmDashboardDiv table:nth-of-type(1) tr.tr_list_header td:nth-of-type(2)&quot;).css(&quot;width&quot;,&quot;40%&quot;);
			$(&quot;#llmDashboardDiv table:nth-of-type(2) td:nth-of-type(2)&quot;).css(&quot;width&quot;,&quot;40%&quot;);
			$(&quot;#llmDashboardDiv table:nth-of-type(2)  td:nth-child(3)&quot;).css(&quot;width&quot;,&quot;50%&quot;);
			//$(&quot;#llmDashboardDiv table:nth-of-type(1) tr.tr_list_header td:nth-of-type(4) b&quot;).css(&quot;padding-left&quot;,&quot;40%&quot;);
			//$(&quot;#llmDashboardDiv table:nth-of-type(1) tr&quot;).not(&quot; , &quot;'&quot; , &quot;:first&quot; , &quot;'&quot; , &quot;).not(&quot;.tr_list_header&quot;).find(&quot;td:nth-of-type(4)&quot;).wrapInner(&quot; , &quot;'&quot; , &quot;&lt;span style=&quot;position:relative;right:100px; text-align: right;display:block&quot;>&lt;/span>&quot; , &quot;'&quot; , &quot;);
			//$(&quot;#llmDashboardDiv table:nth-of-type(1) tr&quot;).not(&quot; , &quot;'&quot; , &quot;:first&quot; , &quot;'&quot; , &quot;).not(&quot;.tr_list_header&quot;).find(&quot;td:nth-of-type(3)&quot;).wrapInner(&quot; , &quot;'&quot; , &quot;&lt;span style=&quot;position:relative;right:59px; text-align: right;display:block&quot;>&lt;/span>&quot; , &quot;'&quot; , &quot;);
			//$(&quot;#llmDashboardDiv table:nth-of-type(2) tr&quot;).not(&quot; , &quot;'&quot; , &quot;:first&quot; , &quot;'&quot; , &quot;).not(&quot;.tr_list_header&quot;).find(&quot;td:nth-of-type(3)&quot;).wrapInner(&quot; , &quot;'&quot; , &quot;&lt;span style=&quot;position:relative;right:42%;text-align: right;display:block;width:72%&quot;>&lt;/span>&quot; , &quot;'&quot; , &quot;);
							var intRegex = /^\d+$/;
							var floatRegex = /^([\d\,\.]*)$/;
							var str1;
							var str = $(&quot; , &quot;'&quot; , &quot;body td&quot; , &quot;'&quot; , &quot;).each(function(){
							var str1 = $(this).text();
							  });
							if ($(&quot;#vetiBtn&quot;).length > 0) {
								$(&quot;.back_image_logo1&quot;).addClass(&quot;menu_add&quot;).find(&quot;img&quot;).attr(&quot;onclick&quot;, &quot;collapsePage()&quot;);
							} else {
								$(&quot;.back_image_logo1&quot;).removeClass(&quot;menu_add&quot;);
							}
							/* if ($.browser.version == 8.0) {
								var a = $.trim($(&quot;.imd_maker_space&quot;).text());
								if (a == &quot;&quot;) {
									/* console.log(&quot;ghkxgf&quot;); 
									$(&quot;.imd_maker_space,.imd_maker_user,.imd_maker_dashbord&quot;)
											.css(&quot;display&quot;, &quot;none&quot;);
									$(&quot;.imd_maker_tab&quot;).css(&quot;width&quot;, &quot;0.1%&quot;);
									$(&quot;.imd_maker_dashbord&quot;).css(&quot;width&quot;,
											&quot;1.4%&quot;);
									$(&quot;.imd_maker_user&quot;).css(&quot;width&quot;, &quot;0.8%&quot;);
									$(
											&quot;.menu-dashboard,.td_under_strip,.menu-save &quot;)
											.css(&quot;width&quot;, &quot;10%&quot;);

								}

							} */

						});
	 
	
		$(&quot;.userr&quot;).click(function() {
			$(this).parents(&quot;.username_icon&quot;).siblings(
			&quot;.username_tooltip&quot;).toggle(&quot;slow&quot;);
		});
		divHeight=&quot;&quot;;
		divHeight1=&quot;&quot;;
		$(window).on(&quot; , &quot;'&quot; , &quot;load&quot; , &quot;'&quot; , &quot;,function(){
			$(&quot;.text_footer&quot;).removeClass(&quot;relativefooter&quot;);
			if($(&quot;body&quot;).css(&quot;overflow-y&quot;)==&quot;scroll&quot;){$(&quot;.text_footer&quot;).addClass(&quot;relativefooter&quot;);}
			else{$(&quot;.text_footer&quot;).removeClass(&quot;relativefooter&quot;);}
    		$(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;margin-top&quot; , &quot;'&quot; , &quot;, divHeight+divHeight1+&quot; , &quot;'&quot; , &quot;px&quot; , &quot;'&quot; , &quot;);
			$(&quot;tr td input&quot;).each(function(){
				if($(this).attr(&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;) == &quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;){
				$(this).addClass(&quot;readonly_text&quot;);  }
			});
			$(&quot;input[name=&quot; , &quot;'&quot; , &quot;addAddressButton&quot; , &quot;'&quot; , &quot;]&quot;).click(function(){
				$(this).parents(&quot;.add_new_address_dtl&quot;).next(&quot;table&quot;).find(&quot;.readonly_text&quot;).removeClass(&quot;readonly_text&quot;);
			});
		});
	
		$(&quot;[name=&quot; , &quot;'&quot; , &quot;prospectListHeading&quot; , &quot;'&quot; , &quot;]&quot;).attr(&quot;id&quot;,&quot;prosListTable&quot;);
		$(&quot;#prosListTable tr td&quot;).eq(2).addClass(&quot;prosListTable_adjust2&quot;);
		$(&quot;#prosListTable tr td&quot;).eq(3).addClass(&quot;prosListTable_adjust3&quot;);
		$(window).on(&quot; , &quot;'&quot; , &quot;load&quot; , &quot;'&quot; , &quot;,function() {
		
			if($(&quot;.tt&quot;).text().trim()==&quot;&quot;){$(&quot;.tt&quot;).fadeOut();}
			if($(&quot;#uu&quot;).text().trim()==&quot;&quot;){$(&quot;#uu&quot;).fadeOut();}
			if($(&quot;#SDNDataId&quot;).text().trim()==&quot;&quot;){$(&quot;#SDNDataId&quot;).fadeOut();}
			$(&quot;#prosListTable tr td&quot;).eq(0).addClass(&quot;prosListTable_adjust1&quot;);
			//$(&quot;#llmDashboardDiv .static_info:nth-of-type(2) tr:nth-of-type(2) td:nth-of-type(3) b&quot;).css(&quot;padding-left&quot;,&quot;23%&quot;);
			//$(&quot;#llmDashboardDiv .static_info:nth-of-type(1) tr:nth-of-type(2) td:nth-of-type(3) b&quot;).css(&quot;padding-left&quot;,&quot;56%&quot;);
			$(&quot;#llmDashboardDiv .static_info:nth-of-type(1) tr td:nth-of-type(1)&quot;).attr(&quot;style&quot;,&quot;width:1%&quot;);
			$(&quot;#llmDashboardDiv table:nth-of-type(1) tr td:nth-of-type(1)&quot;).css(&quot;width&quot;,&quot;3%&quot;);
			$(&quot;#llmDashboardDiv .static_info:nth-of-type(1) tr td:nth-of-type(1) img&quot;).css({&quot;float&quot;:&quot;right&quot;,&quot;margin-right&quot;:&quot;4px&quot;});
			$(&quot;#llmDashboardDiv table:nth-of-type(2) tr td:nth-of-type(1)&quot;).css(&quot;width&quot;,&quot;3%&quot;);
			$(&quot;#llmDashboardDiv .static_info:nth-of-type(2) tr td:nth-of-type(1) img&quot;).css({&quot;float&quot;:&quot;right&quot;,&quot;margin-right&quot;:&quot;4px&quot;});
			$(&quot;.div_applisttable1  tr:not(:first-child)&quot;).each(function(){ 
				var linkhref = $(this).find(&quot;td:nth-child(5) a&quot;).attr(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;);
				$(this).find(&quot;td:nth-child(3)&quot;).wrapInner(&quot; , &quot;'&quot; , &quot;&lt;a href=&quot; , &quot;'&quot; , &quot;+linkhref +&quot; , &quot;'&quot; , &quot;>&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;&lt;/a>&quot; , &quot;'&quot; , &quot;);
			});
			var appname_txt = $(&quot;.applist_imd td:nth-child(3)&quot;).text();
			$(&quot;.applist_imd td:nth-child(3)&quot;).html(appname_txt);
			var divTest = $(&quot;#fset_one&quot;).height();
			var flagg1 = false;
 			$(&quot;.menu_tab tbody td a&quot;).width($(&quot;.menu_tab tbody td a&quot;).parent(&quot;td&quot;).outerWidth()-5);
			$(&quot;#verticalMenu&quot;).css({
							&quot;width&quot; : &quot;94% !important&quot;,
							&quot;position&quot; : &quot;relative&quot;,
							&quot;left&quot; : &quot;143px !important&quot;,
							&quot;overflow-y&quot; : &quot;auto&quot;,
						});

});


	 	
	
	


			








	
			
			
				
					MANUAL ADVICE MAKER
				
			
			
			
    
	
			
		
		 
			My Dashboard
		 
	
		
	        		
		
			
					
				 

			
            
			    
                
				
				
				
				
				
				
				  
				
				
				
				
				
				
				
				  
				   
				   
					
					My Worklist
					
					
					   
				
				 
				 
				 
				
				
				
				
				
				
				
				
				
				
				
				
				
				
			
				
				
				

				
			  
	
				
		    
		    





						
						
						
						
						
						
						
						
						
						
						
						
						
						
						  
						
						
						
						 
						
						
						
						
						
						 Save
						 
						
						
						
						
						
						
						
						
						
						
						
						
						
						
						
						
						
						
						
						
						
						

						
										
								    
								    
						


						
						
						
						
							
						 
						
						 
						 
						
						
						 Save &amp; Exit
							 
							
							
							
							
						
						
						
						

						
						
						
						
						
						


				
			
			
		
		
								
		
	


	      











var inactiveInterval = 1800
var loanAmt = &quot;&quot;;

	//This function is changed by Ravikant for browser compatibility.
	function callWfp()
	{
		var sessionStatus = checkinterval(inactiveInterval);		
		if(sessionStatus==&quot;Y&quot;)
		{	 
			var browser=(navigator.userAgent).toLowerCase();
			//alert(browser);
			if(browser.indexOf(&quot;chrome&quot;)>0)
			{	
				var targetWin = window.open(&quot;workFlowProc.do?actionPerformed=displayWorkFlowPage&amp;rurl=&quot;+Math.random(),&quot;workFlow&quot;,&quot;dialogWidth:1000px;dialogHeight:500px&quot;);
				targetWin.focus();
			}else{
				window.showModalDialog(&quot;workFlowProc.do?actionPerformed=displayWorkFlowPage&amp;rurl=&quot;+Math.random(),&quot;workFlow&quot;,&quot;dialogWidth:1000px;dialogHeight:500px&quot;);
			}
		}
		else
		{
			
			window.location=&quot;userAuthAction.do?dispatchMethod=logout&quot;;
		}
	}
	
	function showNotePad()
	{
		var sessionStatus = checkinterval(inactiveInterval);		
		if(sessionStatus==&quot;Y&quot;)
		{	 
			var browser=(navigator.userAgent).toLowerCase();
			//alert(browser);
			if(browser.indexOf(&quot;chrome&quot;)>0)
			{	
				var targetWin = window.open(&quot;notepadAction.do?actionPerformed=displayNotePadInfo&amp;staticCall=Y&amp;rurl=&quot;+Math.random(),&quot;workFlow&quot;,&quot;dialogWidth:1030px;dialogHeight:600px&quot;);
				targetWin.focus();
			}else{
				window.showModalDialog(&quot;notepadAction.do?actionPerformed=displayNotePadInfo&amp;staticCall=Y&amp;rurl=&quot;+Math.random(),&quot;workFlow&quot;,&quot;dialogWidth:1030px;dialogHeight:600px&quot;);
			}
		}
		else
		{
			
			window.location=&quot;userAuthAction.do?dispatchMethod=logout&quot;;
		}
	}
	
    	/* function for the  Portable Search Div Hide And Show Button(+/-)  date 14 /06/2012  By Ambar Gupta */
	function showdiv()
	{	
	 
           var Lgd = document.getElementById(&quot;searchLgd&quot;);
        //mayank Agrawal for compatiblity
            if (document.getElementById(&quot;one&quot;).style.display == &quot;block&quot; || document.getElementById(&quot;one&quot;).style.display==&quot;&quot;)
            {   
                
                 
                  
                  			   
				    document.getElementById(&quot;imageUpload&quot;).style.display = &quot;block&quot;;
				  
				  
				 
				
				document.getElementById(&quot;one&quot;).style.display = &quot;none&quot;; 
				var temp=document.getElementsByName(&quot;btn_one&quot;);
				
				
				document.getElementById(&quot;fset_one&quot;).style.border= &quot;0px solid #83b0ec&quot;;
				Lgd.innerHTML = &quot; , &quot;'&quot; , &quot;&lt;input type=&quot;button&quot; class=&quot;imageBottonDown&quot;  name=&quot;btn_one&quot; id=&quot;btn_one&quot; onclick=&quot;javascript:showdiv()&quot;  title=&quot;Maximize the Common Information&quot; style=&quot;cursor: pointer;border: none&quot;>&lt;/input>&amp;nbsp;Static Info&quot; , &quot;'&quot; , &quot;;
			}
            else if (document.getElementById(&quot;one&quot;).style.display == &quot;none&quot;)
			{ 
			    
				document.getElementById(&quot;one&quot;).style.display = &quot;block&quot;;
			    
                 
                   
                  			   
				    document.getElementById(&quot;imageUpload&quot;).style.display = &quot;block&quot;;
				  
				  
				 
				
				var temp=document.getElementsByName(&quot;btn_one&quot;);
				
				document.getElementById(&quot;fset_one&quot;).style.border= &quot;1px solid #83b0ec&quot;;
				Lgd.innerHTML = &quot; , &quot;'&quot; , &quot;&lt;input type=&quot;button&quot; class=&quot;imageBottonUp&quot;  name=&quot;btn_one&quot; id=&quot;btn_one&quot;  onclick=&quot;javascript:showdiv()&quot;  title=&quot;Minimize the Common Information&quot; style=&quot;cursor: pointer;border: none&quot;>&lt;/input>&amp;nbsp;Static Info&quot; , &quot;'&quot; , &quot;;
			}	
						
	}
	      /*   Function End Here */	
// 1.0.0.5 start 	      
function rentalPayScheduleReport()
  	{
	window.open(&quot;rentalPayScheduleReport.sprg&quot;);	
  	}	      	
// 1.0.0.5 end 	

//1.0.0.12 start
 function getReschHistScreen()
{
	 
	var paramList=&quot; , &quot;'&quot; , &quot;1000008259&quot; , &quot;'&quot; , &quot;;
    var url =   &quot;dynamicWindow.do?actionPerformed=dynamicWindow&amp;windowId=1000000005&amp;paramList=&quot;+paramList+&quot;&quot;;
	   
	   var self=window.open(url,&quot;dynamicWindow&quot;,&quot;width=800px,height=500px,left=400px,titlebar=yes,scrollbars=yes,toolbar=no,maximize=no,menubar=no,minimize=no,statusbar=no&quot;);
	   
} 
//1.0.0.12 end








 

 


  	   
	

		
				PROSPECT NO	
			DMFIN1000008259
		
			
			
			CUSTCODE		
			CNCIMF000003205
		
		
		
			CUST NAME
			ROHAN  TESTQA
		
		
		
		
		
		
		
		
		CUST LIMIT CODE
			LMCST0000006150
			
			
			
			
		QUOTATION CODE
			QU00022832
			
			
			
			
			
		
			
			GROUP NAME	
			
			PROJECT TEST RO
	    
			
		
			CAPITALISED FLAG
			Y
			
			
			
			
			PRODUCT
			Finance Lease
			
		
			
			SCHEME
			ONLY FIXED FINA...ONLY FIXED FINANCE LEASE
		
		
			
			ASSET CATEGORY
			VEHICLE
		
			
			AGREEMENT TYPE
			BI PARTY
		
		
		
		
		
			SANCTIONED AMT	
			115.00
		
		
		
			
             LEASE SANCTIONE...LEASE SANCTIONED DATE
			13-JAN-2023			
			
		
		
		
		
		
		
			
			
			LEASE RENTAL AMT	
				
			12.00
		
		
			
			RENTAL TYPE
			EQUATED
		
		
		
  
	  
			
			ENTITY TYPE
			INDIVIDUAL
		
			
		
		
		
			
				NO. OF INSTALMENTS
				10
			
		
		
			BRANCH
			HEAD OFFICE
		
	
		
			APPLICATION FOR...APPLICATION FORM NO.
			APPFORM0008259
		
		
		
		
			
			CASHFLOW IRR
			
			5
		
		
		
		
			
			RENTAL START DATE
			
			07-MAR-2023
		
		
		
		
	
	
	
		
		
		
		
			DM STATUS
		
		
		
		
		DISBURSED
		
		
		
		
		
			LEASE APPLICATI...LEASE APPLICATION DATE
		
		
			13-JAN-2023
		
		
		
		
		
		
			
			RENTAL FREQUENCY
			
			MONTHLY
		
		
		
			FORECLOSURE METHOD
			BOOK VALUE
			
			
		
			NPA STAGE
			REGULAR
		
		
		 
	   
    
	
		DAYS SINCE CUST...DAYS SINCE CUSTOMER SIGNED
		
	
	
		
		DAYS IN CURRENT...DAYS IN CURRENT STAGE
		
	
	
	
      
  	
	     
    
				
		
		
	
	    
   
	  
		
	
		
		
		
			TRANSFERED FROM DM
			
			
			
			
			
		
			RV TYPE
			100+1
			
			
			
		
			RV AMOUNT
			                   5.00
			
			
			
			
				
			
			
					    
		    
			
			RESCHEDULED
			
			N
			
			
			
			
			
			
		
			NOTEPAD
			
		
			
		
	
	
	
		
		
		
		   
	
  
  
  
  
  
  
  
 
  
		
			
		
		  
		
 
		   
		   		


 
  
     
     
    
	 
	
	
	
	
	
    



		


	 		 
	  	 




 



	$(document).ready(function(){

		var linkhref = $(&quot;.noti_link span a&quot;).attr(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;);
				$(&quot;.prose2 span&quot;).wrapInner(&quot; , &quot;'&quot; , &quot;&lt;a href=&quot; , &quot;'&quot; , &quot;+linkhref +&quot; , &quot;'&quot; , &quot;>&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;&lt;/a>&quot; , &quot;'&quot; , &quot;);
				
	
				var heightDiv = &quot;80px&quot;;
				
			
				$(&quot;#one&quot;).addClass(&quot;onecustom&quot;);
				$(&quot;#one&quot;).toggleClass(&quot;tgling&quot;).css(&quot;height&quot;,&quot;30px&quot;);
				$(document).on(&quot;click&quot;,&quot;.tgl_nbtn&quot;,function(){
					$(&quot;#one&quot;).toggleClass(&quot;tgling&quot;);
		//1.0.0.4 start			
				if(document.getElementById(&quot;customerLimitStaticInfo&quot;))
				 {
				   heightDiv= Number($(&quot;#customerLimitStaticInfo&quot;).height())+20;
				 }
				 else  if(document.getElementById(&quot;groupStaticInfo&quot;))
				 {
				   heightDiv= Number($(&quot;#groupStaticInfo&quot;).height())+20;
				 }
				  else  if(document.getElementById(&quot;customerStaticInfo&quot;))
				 {
				   heightDiv= Number($(&quot;#customerStaticInfo&quot;).height())+20;
				 }
				 else  if(document.getElementById(&quot;cvLimitStaticInfo&quot;))
				 {
				   heightDiv= Number($(&quot;#cvLimitStaticInfo&quot;).height())+20;
				 }
				 else  if(document.getElementById(&quot;losStaticInfo&quot;))
				 {
				   heightDiv= Number($(&quot;#losStaticInfo&quot;).height())+40;
				 }
			//1.0.0.4 end			
					$(&quot;#one&quot;).css(&quot;height&quot;,heightDiv);
					$(&quot;#one.tgling&quot;).css(&quot;height&quot;,&quot;30px&quot;);
					$(this).toggleClass(&quot;tglingSpan&quot;);
				});

			$(&quot;#fset_one .static_info_head&quot;).each(function(){
			if($(this).text().trim().length>=20) 
			{ 
			var originText1=$(this).text();   
			var cropText1=originText1.substring(0,15); 
			$(this).text(cropText1+&quot;...&quot;); 
			$(this).after(&quot;&lt;span class=&quot; , &quot;'&quot; , &quot;hoverStaticInfo&quot; , &quot;'&quot; , &quot; style=&quot; , &quot;'&quot; , &quot;display:none&quot; , &quot;'&quot; , &quot;>&quot;+originText1+&quot;&lt;/span>&quot;);$(this).addClass(&quot;positioned_div&quot;);
			}
		

});



	
	});

	
		
			


	
		
		
	var contextPath = &quot;/lease&quot;;

		
		
		
		
		
/* All Changes have done by Ambar Gupta for Persist the Stae of menu link and also have Show hide functionalty */
var isCollapsed = false;
//Changed by Ravikant for browser compatibility on 29-Dec-14.
function collapsePage()
{
	var menuDiv = document.getElementById(&quot;verticalMenu&quot;);
	var barDiv = document.getElementById(&quot;linkTD&quot;);
	var vetiTd = document.getElementById(&quot;vetiTd&quot;);
	if(isCollapsed)
	{
		menuDiv.style.display = &quot;block&quot;;
// 		document.getElementById(&quot; , &quot;'&quot; , &quot;dasiTd&quot; , &quot;'&quot; , &quot;).style.width = &quot;80%&quot;;
// 		document.getElementById(&quot; , &quot;'&quot; , &quot;vetiTd&quot; , &quot;'&quot; , &quot;).style.width = &quot;20%&quot;;
		setDimension(&quot;1&quot;);
		
		barDiv.innerHTML = &quot; , &quot;'&quot; , &quot;&amp;nbsp;&amp;nbsp;&amp;nbsp;&amp;nbsp;VERTICAL MENU &amp;nbsp;&amp;nbsp;&amp;nbsp;&amp;nbsp;&amp;nbsp;&amp;nbsp;&amp;nbsp;&amp;nbsp;&lt;input type=&quot;button&quot; class=&quot;imageBottonLeft&quot;  onclick=&quot;collapsePage()&quot; title=&quot;Minimize Vertical Menu&quot; style=&quot;cursor: pointer;border:none;margin-bottom:0px;margin-top:0px;align:right;&quot; id=&quot;vetiBtn&quot;>&quot; , &quot;'&quot; , &quot;;
		isCollapsed = false;
		vetiTd.style.backgroundImage = &quot;&quot;;
	}
	else
	{	
		barDiv.innerHTML = &quot; , &quot;'&quot; , &quot;&lt;input type=&quot;button&quot; class=&quot;imageBottonRight&quot;  onclick=&quot;collapsePage()&quot; title=&quot;Maximize Vertical Menu&quot; style=&quot;cursor: pointer;border: none;margin-bottom:0px;margin-top:0px;&quot; id=&quot;vetiBtn&quot; align=&quot;right&quot;>&quot; , &quot;'&quot; , &quot;;
		menuDiv.style.display = &quot;none&quot;;
// 		document.getElementById(&quot; , &quot;'&quot; , &quot;dasiTd&quot; , &quot;'&quot; , &quot;).style.width = &quot;99%&quot;;
// 		document.getElementById(&quot; , &quot;'&quot; , &quot;vetiTd&quot; , &quot;'&quot; , &quot;).style.width = &quot;1%&quot;;
		setDimension(&quot;2&quot;);
		newImage = &quot;url(images/VERTICALMENUBAR.gif)&quot;;
		vetiTd.style.backgroundImage = newImage;
		isCollapsed = true;
	}
	menuDiv.style.width=&quot;230&quot;;
	//menuDiv.style.height=&quot;100%&quot;;
	menuDiv.style.overflow=&quot;auto&quot;;
}
function changecolor1()
	{
	   var barBtn = document.getElementById(&quot;vetiBtn&quot;);
	   barBtn.style.backgroundColor=&quot; , &quot;'&quot; , &quot;#00FF00&quot; , &quot;'&quot; , &quot;;
	}
	function changecolor()
	{ 
	   
	   var barBtn = document.getElementById(&quot;vetiBtn&quot;);
	   barBtn.style.backgroundColor=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	}
	//Added by Ravikant for browser compatibility on 29-Dec-14.
	function setDimension(d)
	{
		var x = screen.width;
		var y = screen.height;
		if(d == &quot;1&quot;)
		{
			//document.getElementById(&quot;vetiTd&quot;).style.width = (x*0.2) + &quot;px&quot;; 
		   // document.getElementById(&quot;dasiTd&quot;).style.width = (x-(x*0.2)) + &quot;px&quot;;	
		    
		   // document.getElementById(&quot;vetiTd&quot;).style.height = (y-(y*0.01)) + &quot;px&quot;;  
		   // document.getElementById(&quot;dasiTd&quot;).style.height = (y-(y*0.01)) + &quot;px&quot;;   
	    }
	    else if(d == &quot;2&quot;)
	    {
	    	//document.getElementById(&quot;vetiTd&quot;).style.width = (x*0.01) + &quot;px&quot;; 
		   // document.getElementById(&quot;dasiTd&quot;).style.width = (x-(x*0.01)) + &quot;px&quot;;
		    
		   // document.getElementById(&quot;vetiTd&quot;).style.height = (y-(y*0.01)) + &quot;px&quot;;  
		  //  document.getElementById(&quot;dasiTd&quot;).style.height = (y-(y*0.01)) + &quot;px&quot;;  
	    }
	    //alert(document.getElementById(&quot;vetiTd&quot;).style.width+&quot; &quot;+document.getElementById(&quot;dasiTd&quot;).style.width+&quot; &quot;+document.getElementById(&quot;dasiTd&quot;).style.height);
	}

	



		
			

			
				
				
			
			
				
				UTILITYCHANGE PASSWORDCUSTOMERNEWCUSTOMERPENDING ACTIVITIESAPPLICANTDEDUPENOTEPADDOCUMENTCR REVIEWCUSTOMER EDITCUSTOMER VIEWERGROUPNEW GROUPGROUP EDITLIMIT(LEASE)CUSTOMER LIMITNEWCUSTOMER LIMITPENDING ACTIVITIESEDITCR DECISIONCUSTOMER LIMIT VIEWERGROUP LIMITGROUP LIMIT VIEWERQUOTATIONNEWLEASE PROPOSALPENDING ACTIVITIESASSET DETAILSQUOTATIONINSURANCECASHFLOWREGISTRATION DETAILSACCEPTANCEACQUISITIONVIEWERDM QUOTATIONNEWDM APPLICATIONPENDING ACTIVITIESASSET DETAILSQUOTATIONCASHFLOWDM OFFLINE ACTIONDM SANCTIONCAPITALISATION  MAKERDM CANCELLATIONMAKERVIEWERPAYMENTMAKERVIEWERDM VIEWERLMSINVOICEINVOICEDM PDEMAKERRECEIPTMAKERPAYMENT HISTORYRECEIPT CANCELLATIONMAKERVIEWERREFUNDMAKERVIEWERMANUAL ADVICEMAKERKNOCK OFFMAKERVIEWERINSTR MANAGEMENTPDC/ECS GENERATEMAKERPDC/ECS EDITMAKERVIEWERWAIVE OFFMAKERVIEWERCLOSE / FORECLOSEMAKERVIEWERBATCH PROCESSGENERATIONRESCHEDULING CASEMAKERREPORTSCREDIT OPERATION REPORTANNEXES OLAUDIT CONFIRMATIONBOARD OF RESOLUTIONBOARD RESOLUTION CORPORATE GUARANTORCANCELLATION SALARY DEDUCTION FORMCANCELLATION STANDING ORDER FORMCERTIFICAT DE GAGE SECOND HANDCHANGE IN ENGINE NUMBERCHANGE IN REGISTRATION NUMBERCHANGE OF NAME OF HORSEPOWERCORPORATE GUARANTEECOVER LETTERDIRECT DEBIT FORMHORSEPOWER FULLLEASE AGREEMENTLEASE AGREEMENT OLLETTER OF UNDERTAKINGLOST HORSEPOWERNO LIABILITY LETTEROFFER LETTERREPAYMENT SCHEDULERIGHT OF SET OFF LETTERRV LETTERRV SALES DEED AND CERTIFICATE OF GAGESALARY DEDUCTION FORMSALES DEEDSETTLEMENT LETTERSTANDING ORDER FORMSUBORDINATION OF SHAREHOLDERS LOANSUPPLIER LETTERACCOUNTING REPORTACCRUAL REPORTASSET RECEIPT REPORTAUTO RESCHEDULE STATUS REPORTBASE_RATE_CHANGE_TRACK_REPORTCLIENT MIS REPORTCUSTOMER DETAIL REPORTDOCUMENT PENDING AND EXPIRY REPORTINSURANCE REPORTINVOICE REPORTLEAD FOLLOW UPLEAD STATUSLEASE REGISTRATION TRACKINGLIMIT EXPIRYLPI REPORTMATURING AGREEMENTS REPORTOUTSTANDING DUE REPORTRV DUE REPORTSCHEDULED REPORTSTRANSACTION REPORTWAIVER_REPORTMANUAL ADVICELEASE PROPOSALPO REPORTQUOTATION DETAIL REPORTRESCHEDULING REPORTBATCH DTL REPORTSTATEMENT OF ACCOUNTBATCH PRESENTATION REPORTTRANSACTION REPORTTRIAL BALANCEVENDOR PAYMENT REPORTBATCH STATUS REPORTBATCH UPLOAD ISSUES REPORTBULK INVOICE REPORTCOVERNOTE REPORTAGEING REPORTBRANCH WISE SUMMARYUSER WISE SUMMARYCUSTOMER LIMIT REPORTFORECLOSURE REPORTALCO REPORTGROUP CUSTOMER REPORTGROUP LIMIT REPORTLEASE CAM REPORTLEASE CREDIT NOTEVAT INVOICEMASTERSMAKERSALESMANAGERAUTHORBATCH UPLOADMAKERNEGATIVE LISTSDNTALIBANAL-QAIDAMCIBCAUTION LISTAP INT GL MAP CONFIGBULK RECEIPTINSTRUMENT MANAGEMENTLEASE REGISTRATION TRACKINGOFFLINE INSURANCE DETAILSPDC/ECSQT FLEET DETAIL UPLOADLEASE ASSET PRICE MST UPLOADLEASE ASSET VENDOR UPLOADLEASE VENDOR GSTIN UPLOADMANUAL VOUCHER UPLOADCUSTOMER INVOICE CONFIG UPLOADLEASE BATTERY PRICE UPLOADLEASE TYRE PRICE UPLOADMANUAL ADVICE UPLOADLEASE ASSET CESSMAPPING UPLOADLEASE ASSET VARIANT UPLOADLEASE FC AMNT FORMULA UPLOADLEASE SERVICE CHRG AMT UPLOADLEASE VENDOR BANK UPLOADLEASE VENDOR MAPPING UPLOADLEASE MAINT SLABS UPLOADBULK CLOSURECHARGE X GST UPLOADLEASE RESIDUAL VALUE UPLOADAUTHORNEGATIVE LISTCAUTION LISTAP INT GL MAP CONFIGBULK CLOSUREBULK RECEIPTCUSTOMER RECEIPTLEASE ASSET PRICE MST UPLOADPDC/ECSLEASE VENDOR BANK UPLOADMANUAL ADVICE UPLOADLEASE ASSET VARIANT UPLOADLEASE FC AMNT FORMULA UPLOADLEASE SERVICE CHRG AMT UPLOADLEASE ASSET VENDOR UPLOADLEASE VENDOR GSTIN UPLOADMANUAL VOUCHER UPLOAD AUTHORLEASE BATTERY PRICE UPLOADLEASE TYRE PRICE UPLOADLEASE MAINT SLABS UPLOADCHARGE X GST UPLOADLEASE RESIDUAL VALUE UPLOADVIEWERNEGATIVE LISTSDNTALIBANAL-QAIDAMCIBCAUTION LISTAP INT GL MAP CONFIGLEASE REGISTRATION TRACKINGOFFLINE INSURANCE DETAILSLEASE ASSET PRICE MST UPLOADMANUAL VOUCHER UPLOADCUSTOMER INVOICE CONFIG UPLOADMANUAL ADVICE UPLOADLEASE ASSET VARIANT UPLOADLEASE FC AMNT FORMULA UPLOADLEASE SERVICE CHRG AMT UPLOADLEASE VENDOR BANK UPLOADLEASE VENDOR MAPPING UPLOADBULK RECEIPTCUSTOMER RECEIPT VIEWERINSTRUMENT MANAGEMENT VIEWERRESCH VIEWERBULK CLOSURE VIEWERPDC/ECSLEASE ASSET VENDOR UPLOADLEASE VENDOR GSTIN UPLOADLEASE BATTERY PRICE UPLOADLEASE TYRE PRICE UPLOADLEASE MAINT SLABS UPLOADCHARGE X GST UPLOADLEASE RESIDUAL VALUE UPLOAD
					
					var menuDataXml = &quot;&lt;ROWSET> &lt;ROW>  &lt;ACTION_ID>1000000001&lt;/ACTION_ID>  &lt;PARENT_ID/>  &lt;DISPLAY_NAME>UTILITY&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>UTILITY&lt;/MENU_TYPE>  &lt;SEQUENCE>10&lt;/SEQUENCE>  &lt;SCREEN_NAME>UTILITY&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>10&lt;/SEQ_NO>  &lt;PATH>-110&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000002&lt;/ACTION_ID>  &lt;PARENT_ID>1000000001&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CHANGE PASSWORD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>forwardFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000002&amp;amp;screenName=CHANGE PASSWORD&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>CHANGE PASSWORD&lt;/MENU_TYPE>  &lt;SEQUENCE>2&lt;/SEQUENCE>  &lt;SCREEN_NAME>AAA&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>changePassword.do?actionPerformed=displayChangePassword&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>2&lt;/SEQ_NO>  &lt;PATH>-110-22&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000002041&lt;/ACTION_ID>  &lt;PARENT_ID/>  &lt;DISPLAY_NAME>CUSTOMER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>CUSTOMER&lt;/MENU_TYPE>  &lt;SEQUENCE>11&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>11&lt;/SEQ_NO>  &lt;PATH>-111&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000010289&lt;/ACTION_ID>  &lt;PARENT_ID>1000002041&lt;/PARENT_ID>  &lt;DISPLAY_NAME>NEW  &lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>NEW&lt;/MENU_TYPE>  &lt;SEQUENCE>22&lt;/SEQUENCE>  &lt;SCREEN_NAME>NEW&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>22&lt;/SEQ_NO>  &lt;PATH>-111-222&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000002042&lt;/ACTION_ID>  &lt;PARENT_ID>1000010289&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>NewCustomerDetails.do?actionPerformed=displayCustomer&amp;amp;screenId=1000002042&amp;amp;screenName=CUSTOMER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>CUSTOMER&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>NewCustomerDetails.do?actionPerformed=displayCustomer&amp;amp;screenId=1000002042&amp;amp;screenName=CUSTOMER&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>CustomerDetails&lt;/REQUEST_PAGE>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-111-222-33&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000002049&lt;/ACTION_ID>  &lt;PARENT_ID>1000002041&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER EDIT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>applicantListFrmNavAction.do?actionPerformed=displayApplicantListInfo&amp;amp;screenId=1000002049&amp;amp;screenName=CUSTOMER EDIT&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>CUSTOMER&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER EDIT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>applicantList.do?actionPerformed=displayCustomerList&amp;amp;screenId=1000000079&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-111-23&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000002043&lt;/ACTION_ID>  &lt;PARENT_ID>1000002041&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>applicantListFrmNavAction.do?actionPerformed=displayApplicantListInfo&amp;amp;screenId=1000002043&amp;amp;screenName=CUSTOMER VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>CUSTOMER&lt;/MENU_TYPE>  &lt;SEQUENCE>4&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>applicantList.do?actionPerformed=displayCustomerList&amp;amp;screenId=1000002043&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>4&lt;/SEQ_NO>  &lt;PATH>-111-24&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000010288&lt;/ACTION_ID>  &lt;PARENT_ID>1000002041&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PENDING ACTIVITIES &lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PENDING ACTIVITIES&lt;/MENU_TYPE>  &lt;SEQUENCE>5&lt;/SEQUENCE>  &lt;SCREEN_NAME>PENDING ACTIVITIES&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>5&lt;/SEQ_NO>  &lt;PATH>-111-25&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000010283&lt;/ACTION_ID>  &lt;PARENT_ID>1000010288&lt;/PARENT_ID>  &lt;DISPLAY_NAME>APPLICANT &lt;/DISPLAY_NAME>  &lt;ACTION_LINK>applicantListFrmNavAction.do?actionPerformed=displayApplicantListInfo&amp;amp;screenId=1000010283&amp;amp;screenName=CUSTOMER DETAIL&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>APPLICANT DETAIL&lt;/MENU_TYPE>  &lt;SEQUENCE>11&lt;/SEQUENCE>  &lt;SCREEN_NAME>APPLICANT DETAIL&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>applicantList.do?actionPerformed=displayCustomerList&amp;amp;screenId=1000000079&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>11&lt;/SEQ_NO>  &lt;PATH>-111-25-311&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000010284&lt;/ACTION_ID>  &lt;PARENT_ID>1000010288&lt;/PARENT_ID>  &lt;DISPLAY_NAME>DEDUPE &lt;/DISPLAY_NAME>  &lt;ACTION_LINK>applicantListFrmNavAction.do?actionPerformed=displayApplicantListInfo&amp;amp;screenId=1000010284&amp;amp;screenName=CUSTOMER DETAIL&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>DEDUPE&lt;/MENU_TYPE>  &lt;SEQUENCE>12&lt;/SEQUENCE>  &lt;SCREEN_NAME>DEDUPE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>dedupe.do?actionPerformed=displayDedupeCustomerList&amp;amp;screenId=1000000080&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>12&lt;/SEQ_NO>  &lt;PATH>-111-25-312&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000010285&lt;/ACTION_ID>  &lt;PARENT_ID>1000010288&lt;/PARENT_ID>  &lt;DISPLAY_NAME>NOTEPAD &lt;/DISPLAY_NAME>  &lt;ACTION_LINK>applicantListFrmNavAction.do?actionPerformed=displayApplicantListInfo&amp;amp;screenId=1000010285&amp;amp;screenName=CUSTOMER DETAIL&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>NOTEPAD&lt;/MENU_TYPE>  &lt;SEQUENCE>13&lt;/SEQUENCE>  &lt;SCREEN_NAME>NOTEPAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>notepadAction.do?actionPerformed=displayNotePadInfo&amp;amp;screenId=1000000084&amp;amp;mode=E&amp;amp;disName=NOTEPAD&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>13&lt;/SEQ_NO>  &lt;PATH>-111-25-313&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000010287&lt;/ACTION_ID>  &lt;PARENT_ID>1000010288&lt;/PARENT_ID>  &lt;DISPLAY_NAME>DOCUMENT &lt;/DISPLAY_NAME>  &lt;ACTION_LINK>applicantListFrmNavAction.do?actionPerformed=displayApplicantListInfo&amp;amp;screenId=1000010287&amp;amp;screenName=CUSTOMER DETAIL&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>DOCUMENT&lt;/MENU_TYPE>  &lt;SEQUENCE>15&lt;/SEQUENCE>  &lt;SCREEN_NAME>DOCUMENTS&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>documentEntryAction.do?actionPerformed=displayDocumentEntry&amp;amp;screenId=1000000082&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>15&lt;/SEQ_NO>  &lt;PATH>-111-25-315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100000039&lt;/ACTION_ID>  &lt;PARENT_ID>1000010288&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CR REVIEW&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>applicantListFrmNavAction.do?actionPerformed=displayApplicantListInfo&amp;amp;screenId=1100000039&amp;amp;screenName=CR DECISION&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>CR REVIEW&lt;/MENU_TYPE>  &lt;SEQUENCE>5&lt;/SEQUENCE>  &lt;SCREEN_NAME>CREDIT REVIEW&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>persDescAction.do?actionPerformed=displayPersonalDiscussionPage&amp;amp;screenId=1000002044&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>5&lt;/SEQ_NO>  &lt;PATH>-111-25-35&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106515&lt;/ACTION_ID>  &lt;PARENT_ID/>  &lt;DISPLAY_NAME>GROUP&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>GROUP&lt;/MENU_TYPE>  &lt;SEQUENCE>111&lt;/SEQUENCE>  &lt;SCREEN_NAME>GROUP&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>newInsurance&lt;/REQUEST_PAGE>  &lt;SEQ_NO>111&lt;/SEQ_NO>  &lt;PATH>-1111&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106516&lt;/ACTION_ID>  &lt;PARENT_ID>1100106515&lt;/PARENT_ID>  &lt;DISPLAY_NAME>NEW GROUP&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>groupAction.do?actionPerformed=displayGroup&amp;amp;screenId=1100106516&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>GROUP&lt;/MENU_TYPE>  &lt;SEQUENCE>23&lt;/SEQUENCE>  &lt;SCREEN_NAME>NEW GROUP&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>23&lt;/SEQ_NO>  &lt;PATH>-1111-223&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106517&lt;/ACTION_ID>  &lt;PARENT_ID>1100106515&lt;/PARENT_ID>  &lt;DISPLAY_NAME>GROUP EDIT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>groupAction.do?actionPerformed=displayGroup&amp;amp;screenId=1100106517&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>GROUP&lt;/MENU_TYPE>  &lt;SEQUENCE>24&lt;/SEQUENCE>  &lt;SCREEN_NAME>GROUP EDIT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>24&lt;/SEQ_NO>  &lt;PATH>-1111-224&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200006000&lt;/ACTION_ID>  &lt;PARENT_ID/>  &lt;DISPLAY_NAME>LIMIT(LEASE)&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMITS&lt;/MENU_TYPE>  &lt;SEQUENCE>112&lt;/SEQUENCE>  &lt;SCREEN_NAME>LIMIT(LEASE)&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>112&lt;/SEQ_NO>  &lt;PATH>-1112&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200007001&lt;/ACTION_ID>  &lt;PARENT_ID>1200006000&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER LIMIT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMITS&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER LIMIT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-1112-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200006001&lt;/ACTION_ID>  &lt;PARENT_ID>1200006000&lt;/PARENT_ID>  &lt;DISPLAY_NAME>GROUP LIMIT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMITS&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>GROUP LIMIT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-1112-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200007002&lt;/ACTION_ID>  &lt;PARENT_ID>1200007001&lt;/PARENT_ID>  &lt;DISPLAY_NAME>NEW&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMITS&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>NEW&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>customerLimit.do?actionPerformed=displayNewCustomerLimit&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-1112-21-31&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200007037&lt;/ACTION_ID>  &lt;PARENT_ID>1200007002&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER LIMIT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>customerLimit.do?actionPerformed=displayNewCustomerLimit&amp;amp;screenId=1200007037&amp;amp;screenName=CUSTOMER LIMIT&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMITS&lt;/MENU_TYPE>  &lt;SEQUENCE>11&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER LIMIT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>customerLimit.do?actionPerformed=displayNewCustomerLimit&amp;amp;screenId=1200007037&amp;amp;screenName=CUSTOMER LIMIT&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>customerLimit.do?actionPerformed=displayNewCustomerLimit&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>11&lt;/SEQ_NO>  &lt;PATH>-1112-21-31-411&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200007003&lt;/ACTION_ID>  &lt;PARENT_ID>1200007001&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PENDING ACTIVITIES&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMITS&lt;/MENU_TYPE>  &lt;SEQUENCE>2&lt;/SEQUENCE>  &lt;SCREEN_NAME>PENDING ACTIVITIES&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>2&lt;/SEQ_NO>  &lt;PATH>-1112-21-32&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200007004&lt;/ACTION_ID>  &lt;PARENT_ID>1200007003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>EDIT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>customerLimit.do?actionPerformed=customerLimitPendingSearchList&amp;amp;screenId=1200007004&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMITS&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>EDIT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>customerLimit.do?actionPerformed=displayCustomerLimitEdit&amp;amp;screenId=1200007030&amp;amp;screenName=CUSTOMER LIMIT EDIT&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-1112-21-32-41&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200007005&lt;/ACTION_ID>  &lt;PARENT_ID>1200007003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CR DECISION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>customerLimit.do?actionPerformed=customerLimitPendingSearchList&amp;amp;screenId=1200007005&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMITS&lt;/MENU_TYPE>  &lt;SEQUENCE>2&lt;/SEQUENCE>  &lt;SCREEN_NAME>CR DECISION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>creditDecision.do?actionPerformed=displayCreditDecisionPage&amp;amp;screenId=1200007034&amp;amp;screenName=CR DECISION&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>2&lt;/SEQ_NO>  &lt;PATH>-1112-21-32-42&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200006006&lt;/ACTION_ID>  &lt;PARENT_ID>1200006001&lt;/PARENT_ID>  &lt;DISPLAY_NAME>GROUP LIMIT VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>groupLimitAction.do?actionPerformed=groupLimitPendingSearch&amp;amp;screenId=1200006006&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMITS&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>GROUP LIMIT VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>groupLimitAction.do?actionPerformed=displayGroupLimitEdit&amp;amp;screenId=1200006030&amp;amp;screenName=EDIT&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-1112-21-33&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200007006&lt;/ACTION_ID>  &lt;PARENT_ID>1200007001&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER LIMIT VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>customerLimit.do?actionPerformed=customerLimitPendingSearchList&amp;amp;screenId=1200007006&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMITS&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER LIMIT VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>customerLimit.do?actionPerformed=displayCustomerLimitEdit&amp;amp;screenId=1200007030&amp;amp;screenName=EDIT&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-1112-21-33&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003000&lt;/ACTION_ID>  &lt;PARENT_ID/>  &lt;DISPLAY_NAME>QUOTATION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>12&lt;/SEQUENCE>  &lt;SCREEN_NAME>QUOTATION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>12&lt;/SEQ_NO>  &lt;PATH>-112&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003001&lt;/ACTION_ID>  &lt;PARENT_ID>1200003000&lt;/PARENT_ID>  &lt;DISPLAY_NAME>NEW&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>NEW&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospetFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>newProspect&lt;/REQUEST_PAGE>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-112-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003002&lt;/ACTION_ID>  &lt;PARENT_ID>1200003001&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE PROPOSAL&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>newEnquiry.sprg?screenId=1200003002&amp;amp;screenName=NEW ENQUIRY&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>91&lt;/SEQUENCE>  &lt;SCREEN_NAME>NEW ENQUIRY&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>newEnquiry.sprg&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>newProspect&lt;/REQUEST_PAGE>  &lt;SEQ_NO>91&lt;/SEQ_NO>  &lt;PATH>-112-21-391&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003003&lt;/ACTION_ID>  &lt;PARENT_ID>1200003000&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PENDING ACTIVITIES&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>23&lt;/SEQUENCE>  &lt;SCREEN_NAME>PENDING ACTIVITES&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>23&lt;/SEQ_NO>  &lt;PATH>-112-223&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003005&lt;/ACTION_ID>  &lt;PARENT_ID>1200003003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>ASSET DETAILS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>quotationList.sprg?screenId=1200003005&amp;amp;screenName=ASSET&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>ASSET&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>assetDetails.sprg?screenId=1200003012&amp;amp;screenName=ASSET&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-112-223-31&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003006&lt;/ACTION_ID>  &lt;PARENT_ID>1200003003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>QUOTATION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>quotationList.sprg?screenId=1200003006&amp;amp;screenName=QUOTATION&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>11&lt;/SEQUENCE>  &lt;SCREEN_NAME>QUOTATION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>quotationDetails.sprg?screenId=1200003013&amp;amp;screenName=QUOTATION&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>11&lt;/SEQ_NO>  &lt;PATH>-112-223-311&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003007&lt;/ACTION_ID>  &lt;PARENT_ID>1200003003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>INSURANCE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>quotationList.sprg?screenId=1200003007&amp;amp;screenName=INSURANCE&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>4&lt;/SEQUENCE>  &lt;SCREEN_NAME>INSURANCE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>insuranceQuotationDetails.sprg?screenId=1200003014&amp;amp;screenName=INSURANCE&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>4&lt;/SEQ_NO>  &lt;PATH>-112-223-34&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003008&lt;/ACTION_ID>  &lt;PARENT_ID>1200003003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CASHFLOW&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>quotationList.sprg?screenId=1200003008&amp;amp;screenName=CASHFLOW&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>5&lt;/SEQUENCE>  &lt;SCREEN_NAME>CASHFLOW&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>quotationCashflowDetail.sprg?screenId=1200003015&amp;amp;screenName=CASHFLOW&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>5&lt;/SEQ_NO>  &lt;PATH>-112-223-35&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003021&lt;/ACTION_ID>  &lt;PARENT_ID>1200003003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>REGISTRATION DETAILS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>quotationList.sprg?screenId=1200003021&amp;amp;screenName=REGISTRATION DETAILS&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>6&lt;/SEQUENCE>  &lt;SCREEN_NAME>REGISTRATION DETAILS&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>quotationVroDetail.sprg?&amp;amp;screenId=1200003022&amp;amp;screenName=REGISTRATION DETAILS&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>6&lt;/SEQ_NO>  &lt;PATH>-112-223-36&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003009&lt;/ACTION_ID>  &lt;PARENT_ID>1200003003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>ACCEPTANCE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>quotationList.sprg?screenId=1200003009&amp;amp;screenName=ACCEPTANCE&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>7&lt;/SEQUENCE>  &lt;SCREEN_NAME>ACCEPTANCE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>acceptanceDetails.sprg?screenId=1200003016&amp;amp;screenName=ACCEPTANCE&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>7&lt;/SEQ_NO>  &lt;PATH>-112-223-37&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003010&lt;/ACTION_ID>  &lt;PARENT_ID>1200003003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>ACQUISITION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>quotationList.sprg?screenId=1200003010&amp;amp;screenName=ACQUISITION&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>8&lt;/SEQUENCE>  &lt;SCREEN_NAME>ACQUISITION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>acquisitionDetails.sprg?screenId=1200003017&amp;amp;screenName=ACQUISITION&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>8&lt;/SEQ_NO>  &lt;PATH>-112-223-38&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003020&lt;/ACTION_ID>  &lt;PARENT_ID>1200003000&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>quotationList.sprg?screenId=1200003020&amp;amp;screenName=VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>24&lt;/SEQUENCE>  &lt;SCREEN_NAME>VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>quotationDetails.sprg?screenId=1200003013&amp;amp;screenName=QUOTATION&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>24&lt;/SEQ_NO>  &lt;PATH>-112-224&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004000&lt;/ACTION_ID>  &lt;PARENT_ID/>  &lt;DISPLAY_NAME>DM QUOTATION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>13&lt;/SEQUENCE>  &lt;SCREEN_NAME>DM_LOS_LEASE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>13&lt;/SEQ_NO>  &lt;PATH>-113&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004001&lt;/ACTION_ID>  &lt;PARENT_ID>1200004000&lt;/PARENT_ID>  &lt;DISPLAY_NAME>NEW &lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>NEW_DM_APPLICATION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-113-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004002&lt;/ACTION_ID>  &lt;PARENT_ID>1200004001&lt;/PARENT_ID>  &lt;DISPLAY_NAME>DM APPLICATION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>dMQuotationApplicantDetail.sprg?screenId=1200003002&amp;amp;screenName=NEW DM APPLICATION&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>2&lt;/SEQUENCE>  &lt;SCREEN_NAME>DM_APPLICATION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>dMQuotationApplicantDetail.sprg&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>newProspect&lt;/REQUEST_PAGE>  &lt;SEQ_NO>2&lt;/SEQ_NO>  &lt;PATH>-113-21-32&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004003&lt;/ACTION_ID>  &lt;PARENT_ID>1200004000&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PENDING ACTIVITIES&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>2&lt;/SEQUENCE>  &lt;SCREEN_NAME>PENDING ACTIVITES&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>2&lt;/SEQ_NO>  &lt;PATH>-113-22&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004005&lt;/ACTION_ID>  &lt;PARENT_ID>1200004003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>ASSET DETAILS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>dmProspectList.sprg?screenId=1200004005&amp;amp;screenName=ASSET&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>2&lt;/SEQUENCE>  &lt;SCREEN_NAME>ASSET&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>assetDetails.sprg?screenId=1200004019&amp;amp;screenName=ASSET&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>2&lt;/SEQ_NO>  &lt;PATH>-113-22-32&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004006&lt;/ACTION_ID>  &lt;PARENT_ID>1200004003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>QUOTATION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>dmProspectList.sprg?screenId=1200004006&amp;amp;screenName=QUOTATION&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>QUOTATION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>quotationDetails.sprg?screenId=1200004020&amp;amp;screenName=QUOTATION&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-113-22-33&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004008&lt;/ACTION_ID>  &lt;PARENT_ID>1200004003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CASHFLOW&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>dmProspectList.sprg?screenId=1200004008&amp;amp;screenName=CASHFLOW&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>5&lt;/SEQUENCE>  &lt;SCREEN_NAME>CASHFLOW&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>quotationCashflowDetail.sprg?screenId=1200004022&amp;amp;screenName=CASHFLOW&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>5&lt;/SEQ_NO>  &lt;PATH>-113-22-35&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004013&lt;/ACTION_ID>  &lt;PARENT_ID>1200004003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>DM OFFLINE ACTION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>dmProspectList.sprg?screenId=1200004013&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>6&lt;/SEQUENCE>  &lt;SCREEN_NAME>DM OFFLINE ACTION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>dmWorkflow.sprg?screenId=1200004016&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>6&lt;/SEQ_NO>  &lt;PATH>-113-22-36&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004009&lt;/ACTION_ID>  &lt;PARENT_ID>1200004003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>DM SANCTION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>dmProspectList.sprg?screenId=1200004009&amp;amp;screenName=DM SANCTION&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>6&lt;/SEQUENCE>  &lt;SCREEN_NAME>DM SANCTION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>creditDecision.do?actionPerformed=displayCreditDecisionPage&amp;amp;screenId=1000000086&amp;amp;screenName=DM SANCTION&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>6&lt;/SEQ_NO>  &lt;PATH>-113-22-36&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004010&lt;/ACTION_ID>  &lt;PARENT_ID>1200004003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CAPITALISATION  MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>dmProspectList.sprg?screenId=1200004010&amp;amp;screenName=DISBURSAL_MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>7&lt;/SEQUENCE>  &lt;SCREEN_NAME>CAPITALISATION  MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>disbursalFrmNav.do?actionPerformed=displayDMDisbursalInfo&amp;amp;screenId=1200004017&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>7&lt;/SEQ_NO>  &lt;PATH>-113-22-37&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004023&lt;/ACTION_ID>  &lt;PARENT_ID>1200004000&lt;/PARENT_ID>  &lt;DISPLAY_NAME>DM CANCELLATION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LOAN CANCELATION&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>DM CANCELLATION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-113-23&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004012&lt;/ACTION_ID>  &lt;PARENT_ID>1200004000&lt;/PARENT_ID>  &lt;DISPLAY_NAME>DM VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>dmProspectList.sprg?screenId=1200004012&amp;amp;screenName=VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>DM VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>quotationDetails.sprg?screenId=1200004020&amp;amp;screenName=QUOTATION&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-113-23&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004024&lt;/ACTION_ID>  &lt;PARENT_ID>1200004023&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>dmProspectList.sprg?screenId=1200004024&amp;amp;screenName=CANCELLATION MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LOAN CANCELATION&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>DM CANCELLATION MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>disbCancellationAction.do?actionPerformed=displayDmCancellation&amp;amp;screenId=1200004027&amp;amp;screenName=CANCELLATION MAKER&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-113-23-31&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004026&lt;/ACTION_ID>  &lt;PARENT_ID>1200004023&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>dmProspectList.sprg?screenId=1200004026&amp;amp;screenName=CANCELLATION VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LOAN CANCELATION&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>DM CANCELLATION VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>disbCancellationAction.do?actionPerformed=displayDmCancellation&amp;amp;screenId=1200004026&amp;amp;screenName=CANCELLATION VIEWER&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-113-23-33&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004029&lt;/ACTION_ID>  &lt;PARENT_ID>1200004000&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PAYMENT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PAYMENT&lt;/MENU_TYPE>  &lt;SEQUENCE>4&lt;/SEQUENCE>  &lt;SCREEN_NAME>PAYMENT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>4&lt;/SEQ_NO>  &lt;PATH>-113-24&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004030&lt;/ACTION_ID>  &lt;PARENT_ID>1200004029&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>forwardFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1200004030&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PAYMENT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>PAYMENT MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>paymentAction.do?actionPerformed=displayPaymentScreen&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-113-24-31&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004032&lt;/ACTION_ID>  &lt;PARENT_ID>1200004029&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>forwardFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1200004032&amp;amp;screenName=VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PAYMENT&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>PAYMENT VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>paymentAction.do?actionPerformed=displayPaymentScreen&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-113-24-33&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000017&lt;/ACTION_ID>  &lt;PARENT_ID/>  &lt;DISPLAY_NAME>LMS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LMS&lt;/MENU_TYPE>  &lt;SEQUENCE>14&lt;/SEQUENCE>  &lt;SCREEN_NAME>LMS&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>14&lt;/SEQ_NO>  &lt;PATH>-114&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106854&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>INVOICE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>INVOICE&lt;/MENU_TYPE>  &lt;SEQUENCE>31&lt;/SEQUENCE>  &lt;SCREEN_NAME>INVOICE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>31&lt;/SEQ_NO>  &lt;PATH>-114-231&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106856&lt;/ACTION_ID>  &lt;PARENT_ID>1200106854&lt;/PARENT_ID>  &lt;DISPLAY_NAME>INVOICE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>forwardFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1200106856&amp;amp;screenName=INVOICE&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>INVOICE&lt;/MENU_TYPE>  &lt;SEQUENCE>50&lt;/SEQUENCE>  &lt;SCREEN_NAME>INVOICE&lt;/SCREEN_NAME>  &lt;AUTHMODE/>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>invoiceDetails.sprg?screenId=1200106856&amp;amp;screenName=INVOICE&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>50&lt;/SEQ_NO>  &lt;PATH>-114-231-350&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106858&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>DM PDE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>DM PDE&lt;/MENU_TYPE>  &lt;SEQUENCE>32&lt;/SEQUENCE>  &lt;SCREEN_NAME>DM PDE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>32&lt;/SEQ_NO>  &lt;PATH>-114-232&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106859&lt;/ACTION_ID>  &lt;PARENT_ID>1200106858&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1200106859&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>DM PDE&lt;/MENU_TYPE>  &lt;SEQUENCE>33&lt;/SEQUENCE>  &lt;SCREEN_NAME>DM PDE MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>pdeApplicantList.do?actionPerformed=displayPDECustomerList&amp;amp;screenId=1000000090&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>33&lt;/SEQ_NO>  &lt;PATH>-114-232-333&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000034&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>RECEIPT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>OTC&lt;/MENU_TYPE>  &lt;SEQUENCE>42&lt;/SEQUENCE>  &lt;SCREEN_NAME>OTC&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>42&lt;/SEQ_NO>  &lt;PATH>-114-242&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000036&lt;/ACTION_ID>  &lt;PARENT_ID>1000000034&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000036&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>OTC&lt;/MENU_TYPE>  &lt;SEQUENCE>44&lt;/SEQUENCE>  &lt;SCREEN_NAME>RECEIPT MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>otcFrmNavAction.do?actionPerformed=displayOtcInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/lms/otcFrmNavAction.do?actionPerformed=displayOtcInfo&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>44&lt;/SEQ_NO>  &lt;PATH>-114-242-344&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000038&lt;/ACTION_ID>  &lt;PARENT_ID>1000000034&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PAYMENT HISTORY&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000038&amp;amp;screenName=VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>OTC&lt;/MENU_TYPE>  &lt;SEQUENCE>46&lt;/SEQUENCE>  &lt;SCREEN_NAME>PAYMENT HISTORY&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>viewerAction.do?actionPerformed=displayViewerInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>otcFrmNavAction.do?actionPerformed=displayOtcInfo&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>46&lt;/SEQ_NO>  &lt;PATH>-114-242-346&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000010034&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>RECEIPT CANCELLATION &lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>RECEIPT_CANCELLATION &lt;/MENU_TYPE>  &lt;SEQUENCE>44&lt;/SEQUENCE>  &lt;SCREEN_NAME>RECEIPT_CANCELLATION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>44&lt;/SEQ_NO>  &lt;PATH>-114-244&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000010035&lt;/ACTION_ID>  &lt;PARENT_ID>1000010034&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000010035&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>RECEIPT_CANCELLATION &lt;/MENU_TYPE>  &lt;SEQUENCE>44&lt;/SEQUENCE>  &lt;SCREEN_NAME>RECEIPT CANCELLATION MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>receipt_cancellationFrmNavAction.do?actionPerformed=displayreceiptInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/lms/otcFrmNavAction.do?actionPerformed=displayOtcInfo&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>44&lt;/SEQ_NO>  &lt;PATH>-114-244-344&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000010109&lt;/ACTION_ID>  &lt;PARENT_ID>1000010034&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000010109&amp;amp;screenName=VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>RECEIPT CANCELLATION&lt;/MENU_TYPE>  &lt;SEQUENCE>46&lt;/SEQUENCE>  &lt;SCREEN_NAME>RECEIPT CANCELLATION VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>viewerAction.do?actionPerformed=displayViewerInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>otcFrmNavAction.do?actionPerformed=displayOtcInfo&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>46&lt;/SEQ_NO>  &lt;PATH>-114-244-346&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000039&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>REFUND&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REFUND&lt;/MENU_TYPE>  &lt;SEQUENCE>47&lt;/SEQUENCE>  &lt;SCREEN_NAME>REFUND&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>47&lt;/SEQ_NO>  &lt;PATH>-114-247&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000041&lt;/ACTION_ID>  &lt;PARENT_ID>1000000039&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000041&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REFUND&lt;/MENU_TYPE>  &lt;SEQUENCE>49&lt;/SEQUENCE>  &lt;SCREEN_NAME>REFUND MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>refundFrmNavAction.do?actionPerformed=displayRefundInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>49&lt;/SEQ_NO>  &lt;PATH>-114-247-349&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000043&lt;/ACTION_ID>  &lt;PARENT_ID>1000000039&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000043&amp;amp;screenName=VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REFUND&lt;/MENU_TYPE>  &lt;SEQUENCE>51&lt;/SEQUENCE>  &lt;SCREEN_NAME>REFUND VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>viewerAction.do?actionPerformed=displayViewerInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>51&lt;/SEQ_NO>  &lt;PATH>-114-247-351&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001980&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MANUAL ADVICE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>MANUAL ADVICE&lt;/MENU_TYPE>  &lt;SEQUENCE>48&lt;/SEQUENCE>  &lt;SCREEN_NAME>MANUAL ADVICE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>48&lt;/SEQ_NO>  &lt;PATH>-114-248&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001981&lt;/ACTION_ID>  &lt;PARENT_ID>1000001980&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000001981&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>MANUAL ADVICE&lt;/MENU_TYPE>  &lt;SEQUENCE>10&lt;/SEQUENCE>  &lt;SCREEN_NAME>MANUAL ADVICE MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>manualAdviceAction.do?actionPerformed=displayManualAdviceInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/lms/manualAdviceFrmNavAction.do?actionPerformed=displayManualAdviceInfo&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>10&lt;/SEQ_NO>  &lt;PATH>-114-248-310&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000054&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>KNOCK OFF&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>KNOCK OFF&lt;/MENU_TYPE>  &lt;SEQUENCE>57&lt;/SEQUENCE>  &lt;SCREEN_NAME>KNOCK OFF&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>57&lt;/SEQ_NO>  &lt;PATH>-114-257&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000056&lt;/ACTION_ID>  &lt;PARENT_ID>1000000054&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000056&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>KNOCK OFF&lt;/MENU_TYPE>  &lt;SEQUENCE>59&lt;/SEQUENCE>  &lt;SCREEN_NAME>KNOCK OFF MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>knockOffFrmNavAction.do?actionPerformed=displayKnockOffInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>59&lt;/SEQ_NO>  &lt;PATH>-114-257-359&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000058&lt;/ACTION_ID>  &lt;PARENT_ID>1000000054&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000058&amp;amp;screenName=VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>KNOCK OFF&lt;/MENU_TYPE>  &lt;SEQUENCE>61&lt;/SEQUENCE>  &lt;SCREEN_NAME>KNOCK OFF VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>viewerAction.do?actionPerformed=displayViewerInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>knockOffFrmNavAction.do?actionPerformed=displayKnockOffInfo&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>61&lt;/SEQ_NO>  &lt;PATH>-114-257-361&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000024&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>INSTR MANAGEMENT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>INSTRUMENT MANAGEMENT&lt;/MENU_TYPE>  &lt;SEQUENCE>59&lt;/SEQUENCE>  &lt;SCREEN_NAME>INSTRUMENT MANAGEMENT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>59&lt;/SEQ_NO>  &lt;PATH>-114-259&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000025&lt;/ACTION_ID>  &lt;PARENT_ID>1000000024&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PDC/ECS GENERATE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3_1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PDC_GENERATE&lt;/MENU_TYPE>  &lt;SEQUENCE>33&lt;/SEQUENCE>  &lt;SCREEN_NAME>PDC/ECS GENERATE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>33&lt;/SEQ_NO>  &lt;PATH>-114-259-333&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000026&lt;/ACTION_ID>  &lt;PARENT_ID>1000000025&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000026&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PDC_GENERATE&lt;/MENU_TYPE>  &lt;SEQUENCE>35&lt;/SEQUENCE>  &lt;SCREEN_NAME>PDC/ECS GENERATE MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>pdcEcsFrmNavAction.do?actionPerformed=displayPdcEcsInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/lms/pdcEcsFrmNavAction.do?actionPerformed=displayPdcEcsInfo&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>35&lt;/SEQ_NO>  &lt;PATH>-114-259-333-435&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000029&lt;/ACTION_ID>  &lt;PARENT_ID>1000000024&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PDC/ECS EDIT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3_1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PDC_EDIT&lt;/MENU_TYPE>  &lt;SEQUENCE>37&lt;/SEQUENCE>  &lt;SCREEN_NAME>PDC/ECS EDIT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>37&lt;/SEQ_NO>  &lt;PATH>-114-259-337&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000031&lt;/ACTION_ID>  &lt;PARENT_ID>1000000029&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000031&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PDC_EDIT&lt;/MENU_TYPE>  &lt;SEQUENCE>39&lt;/SEQUENCE>  &lt;SCREEN_NAME>PDC/ECS EDIT MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>pdcEcsEditFrmNavAction.do?actionPerformed=displayPdcEcsEditInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/lms/pdcEcsEditFrmNavAction.do?actionPerformed=displayPdcEcsEditInfo&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>39&lt;/SEQ_NO>  &lt;PATH>-114-259-337-439&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000033&lt;/ACTION_ID>  &lt;PARENT_ID>1000000029&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000033&amp;amp;screenName=VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PDC_EDIT&lt;/MENU_TYPE>  &lt;SEQUENCE>41&lt;/SEQUENCE>  &lt;SCREEN_NAME>PDC/ECS EDIT VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>viewerAction.do?actionPerformed=displayViewerInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>pdcEcsEditFrmNavAction.do?actionPerformed=displayPdcEcsEditInfo&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>41&lt;/SEQ_NO>  &lt;PATH>-114-259-337-441&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000049&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>WAIVE OFF&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>WAIVEOFF&lt;/MENU_TYPE>  &lt;SEQUENCE>60&lt;/SEQUENCE>  &lt;SCREEN_NAME>WAIVEOFF&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>60&lt;/SEQ_NO>  &lt;PATH>-114-260&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000051&lt;/ACTION_ID>  &lt;PARENT_ID>1000000049&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000051&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>WAIVE&lt;/MENU_TYPE>  &lt;SEQUENCE>54&lt;/SEQUENCE>  &lt;SCREEN_NAME>WAIVE MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>waiveOffFrmNavAction.do?actionPerformed=displayWaiveOffInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>54&lt;/SEQ_NO>  &lt;PATH>-114-260-354&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000053&lt;/ACTION_ID>  &lt;PARENT_ID>1000000049&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000053&amp;amp;screenName=VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>WAIVE&lt;/MENU_TYPE>  &lt;SEQUENCE>56&lt;/SEQUENCE>  &lt;SCREEN_NAME>WAIVE VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>viewerAction.do?actionPerformed=displayViewerInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>waiveOffFrmNavAction.do?actionPerformed=displayWaiveOffInfo&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>56&lt;/SEQ_NO>  &lt;PATH>-114-260-356&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000059&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CLOSE / FORECLOSE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>FORECLOSURE&lt;/MENU_TYPE>  &lt;SEQUENCE>62&lt;/SEQUENCE>  &lt;SCREEN_NAME>CLOSE/FORECLOSE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>62&lt;/SEQ_NO>  &lt;PATH>-114-262&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000061&lt;/ACTION_ID>  &lt;PARENT_ID>1000000059&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000061&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>FORECLOSURE&lt;/MENU_TYPE>  &lt;SEQUENCE>64&lt;/SEQUENCE>  &lt;SCREEN_NAME>CLOSE/FORECLOSE MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>foreClouserFrmNavAction.do?actionPerformed=displayForeClouserInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>64&lt;/SEQ_NO>  &lt;PATH>-114-262-364&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000063&lt;/ACTION_ID>  &lt;PARENT_ID>1000000059&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000063&amp;amp;screenName=VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>FORECLOSURE&lt;/MENU_TYPE>  &lt;SEQUENCE>66&lt;/SEQUENCE>  &lt;SCREEN_NAME>CLOSE/FORECLOSE VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>foreClouserFrmNavAction.do?actionPerformed=displayForeClouserInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>66&lt;/SEQ_NO>  &lt;PATH>-114-262-366&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000064&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BATCH PROCESS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH PROCESS&lt;/MENU_TYPE>  &lt;SEQUENCE>67&lt;/SEQUENCE>  &lt;SCREEN_NAME>BATCH PROCESS&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>67&lt;/SEQ_NO>  &lt;PATH>-114-267&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000065&lt;/ACTION_ID>  &lt;PARENT_ID>1000000064&lt;/PARENT_ID>  &lt;DISPLAY_NAME>GENERATION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>forwardFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000065&amp;amp;screenName=GENERATION&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3_1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_GENERATION&lt;/MENU_TYPE>  &lt;SEQUENCE>68&lt;/SEQUENCE>  &lt;SCREEN_NAME>GENERATION&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>batchGenerationFrmNavAction.do?actionPerformed=displayBatchProcessInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>68&lt;/SEQ_NO>  &lt;PATH>-114-267-368&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001045&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>RESCHEDULING CASE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>RESCHEDULE  CASE&lt;/MENU_TYPE>  &lt;SEQUENCE>82&lt;/SEQUENCE>  &lt;SCREEN_NAME>RESCHEDULE CASE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>82&lt;/SEQ_NO>  &lt;PATH>-114-282&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001046&lt;/ACTION_ID>  &lt;PARENT_ID>1000001045&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000001046&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>RESCHEDULE  CASE&lt;/MENU_TYPE>  &lt;SEQUENCE>83&lt;/SEQUENCE>  &lt;SCREEN_NAME>RESCHEDULE CASE MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>rescheduleCaseFrmNavAction.do?actionPerformed=displayRescheduleCaseData&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>83&lt;/SEQ_NO>  &lt;PATH>-114-282-383&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000074&lt;/ACTION_ID>  &lt;PARENT_ID/>  &lt;DISPLAY_NAME>REPORTS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>17&lt;/SEQUENCE>  &lt;SCREEN_NAME>REPORTS&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>17&lt;/SEQ_NO>  &lt;PATH>-117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000010249&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>ACCOUNTING REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1000010249&amp;amp;actionId=1000010249&amp;amp;mode=R&amp;amp;actionName=ACCOUNTING_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>ACCOUNTING_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>ACCOUNTING_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1000010249\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109018&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>ACCRUAL REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109018&amp;amp;actionId=1200109018&amp;amp;mode=R&amp;amp;actionName=ACCRUAL_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>ACCRUAL_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>ACCRUAL REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109018\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109019&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>ASSET RECEIPT REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109019&amp;amp;actionId=1200109019&amp;amp;mode=R&amp;amp;actionName=ASSET_RECEIPT_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>ASSET_RECEIPT_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>ASSET RECEIPT REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109019\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106909&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>AUTO RESCHEDULE STATUS REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1100106909&amp;amp;actionId=1100106909&amp;amp;mode=R&amp;amp;actionName=AUTO_RESCHEDULE_STATUS_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>AUTO_RESCHEDULE_STATUS_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>AUTO_RESCHEDULE_STATUS_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1100106909\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106908&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BASE_RATE_CHANGE_TRACK_REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1100106908&amp;amp;actionId=1100106908&amp;amp;mode=R&amp;amp;actionName=BASE_RATE_CHANGE_TRACK_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BASE_RATE_CHANGE_TRACK_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>BASE_RATE_CHANGE_TRACK_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1100106908\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109049&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CLIENT MIS REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109049&amp;amp;actionId=1200109049&amp;amp;mode=R&amp;amp;actionName=CLIENT_MIS_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>CLIENT MIS REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109049\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109103&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER DETAIL REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109103&amp;amp;actionId=1200109103&amp;amp;mode=R&amp;amp;actionName=CUSTOMER_DETAIL_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>CUSTOMER DETAIL REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER DETAIL REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109103\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109098&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>DOCUMENT PENDING AND EXPIRY REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109098&amp;amp;actionId=1200109098&amp;amp;mode=R&amp;amp;actionName=DOCUMENT_EXPIRY&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>DOCUMENT EXPIRY&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>DOCUMENT PENDING AND EXPIRY REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109097\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109124&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>INSURANCE REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109124&amp;amp;actionId=1200109124&amp;amp;mode=R&amp;amp;actionName=INSURANCE_EXPIRY_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>INSURANCE REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109124\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109010&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>INVOICE REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109010&amp;amp;actionId=1200109010&amp;amp;mode=R&amp;amp;actionName=INVOICE_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>INVOICE_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>INVOICE REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109010\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109090&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEAD FOLLOW UP&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109090&amp;amp;actionId=1200109090&amp;amp;mode=R&amp;amp;actionName=LEAD_FOLLOW_UP&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LEAD FOLLOW UP&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEAD FOLLOW UP&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109090\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109037&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEAD STATUS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109037&amp;amp;actionId=1200109037&amp;amp;mode=R&amp;amp;actionName=LEAD_STATUS&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LEAD STATUS&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEAD STATUS&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109037\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109169&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE REGISTRATION TRACKING&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109169&amp;amp;actionId=1200109169&amp;amp;mode=R&amp;amp;actionName=LEASE_REGISTRATION_TRACKING&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LEASE REGISTRATION TRACKING&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE REGISTRATION TRACKING&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109169\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109097&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LIMIT EXPIRY&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109097&amp;amp;actionId=1200109097&amp;amp;mode=R&amp;amp;actionName=LIMIT_EXPIRY&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMIT EXPIRY&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>LIMIT EXPIRY&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109097\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109128&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LPI REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109128&amp;amp;actionId=1200109128&amp;amp;mode=R&amp;amp;actionName=LPI REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>LPI REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109128\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109170&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MATURING AGREEMENTS REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109170&amp;amp;actionId=1200109170&amp;amp;mode=R&amp;amp;actionName=MATURING_AGREEMENTS_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>MATURING AGREEMENTS REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>MATURING AGREEMENTS&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109170\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109030&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>OUTSTANDING DUE REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109030&amp;amp;actionId=1200109030&amp;amp;mode=R&amp;amp;actionName=OUTSTANDING_DUE_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>OUTSTANDING_DUE_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>OUTSTANDING DUE REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109030\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109029&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>RV DUE REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109029&amp;amp;actionId=1200109029&amp;amp;mode=R&amp;amp;actionName=RV_DUE_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>RV_DUE_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>RV DUE REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109029\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109062&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>SCHEDULED REPORTS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109062&amp;amp;actionId=1200109062&amp;amp;mode=R&amp;amp;actionName=SCHEDULED_REPORTS&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>SCHEDULED REPORTS&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109062\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109017&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>TRANSACTION REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109017&amp;amp;actionId=1200109017&amp;amp;mode=R&amp;amp;actionName=TRANSACTION_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>TRANSACTION_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>TRANSACTION REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109017\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109125&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>WAIVER_REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109125&amp;amp;actionId=1200109125&amp;amp;mode=R&amp;amp;actionName=WAIVER_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>WAIVER_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109125\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108937&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MANUAL ADVICE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108937&amp;amp;actionId=1200108937&amp;amp;mode=R&amp;amp;actionName=MANUAL_ADVICE&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>MANUAL_ADVICE&lt;/MENU_TYPE>  &lt;SEQUENCE>105&lt;/SEQUENCE>  &lt;SCREEN_NAME>MANUAL_ADVICE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200108937\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>105&lt;/SEQ_NO>  &lt;PATH>-117-2105&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109133&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE PROPOSAL&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109133&amp;amp;actionId=1200109133&amp;amp;mode=R&amp;amp;actionName=LEASE_PROPOSAL&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_PROPOSAL&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109133&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-2117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106732&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PO REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200106732&amp;amp;actionId=1200106732&amp;amp;mode=R&amp;amp;actionName=PO_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PO_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>PO_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200106732&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-2117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108995&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>QUOTATION DETAIL REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108995&amp;amp;actionId=1200108995&amp;amp;mode=R&amp;amp;actionName=QUOTATION_DETAIL_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION_DETAIL_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>121&lt;/SEQUENCE>  &lt;SCREEN_NAME>QUOTATION DETAIL REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200108995\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>121&lt;/SEQ_NO>  &lt;PATH>-117-2121&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100010146&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>RESCHEDULING REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1100010146&amp;amp;actionId=1100010146&amp;amp;mode=R&amp;amp;actionName=RESCHEDULING_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>RESCHEDULING_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>129&lt;/SEQUENCE>  &lt;SCREEN_NAME>RESCHEDULING_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1100010146\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>129&lt;/SEQ_NO>  &lt;PATH>-117-2129&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106857&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BATCH DTL REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200106857&amp;amp;actionId=1200106857&amp;amp;mode=R&amp;amp;actionName=BATCH_DTL_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_DTL_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>13&lt;/SEQUENCE>  &lt;SCREEN_NAME>BATCH_DTL_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200106857\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>13&lt;/SEQ_NO>  &lt;PATH>-117-213&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106489&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>STATEMENT OF ACCOUNT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1100106489&amp;amp;actionId=1100106489&amp;amp;mode=R&amp;amp;actionName=SOA_NEW&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>137&lt;/SEQUENCE>  &lt;SCREEN_NAME>STATEMENT_OF_ACCOUNT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1100106489\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>137&lt;/SEQ_NO>  &lt;PATH>-117-2137&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106727&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BATCH PRESENTATION REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1100106727&amp;amp;actionId=1100106727&amp;amp;mode=R&amp;amp;actionName=BATCH_PRESENTATION_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_PRESENTATION_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>14&lt;/SEQUENCE>  &lt;SCREEN_NAME>BATCH_PRESENTATION_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1100106727\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>14&lt;/SEQ_NO>  &lt;PATH>-117-214&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106510&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>TRANSACTION REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1100106510&amp;amp;actionId=1100106510&amp;amp;mode=R&amp;amp;actionName=TRANSACTION_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>TRANSACTION_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>140&lt;/SEQUENCE>  &lt;SCREEN_NAME>TRANSACTION REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1100106510\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>140&lt;/SEQ_NO>  &lt;PATH>-117-2140&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106511&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>TRIAL BALANCE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1100106511&amp;amp;actionId=1100106511&amp;amp;mode=R&amp;amp;actionName=TRIAL_BALANCE&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>TRIAL_BALANCE&lt;/MENU_TYPE>  &lt;SEQUENCE>141&lt;/SEQUENCE>  &lt;SCREEN_NAME>TRIAL BALANCE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1100106511\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>141&lt;/SEQ_NO>  &lt;PATH>-117-2141&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108988&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VENDOR PAYMENT REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108988&amp;amp;actionId=1200108988&amp;amp;mode=R&amp;amp;actionName=VENDOR_PAYMENT_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>VENDOR_PAYMENT_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>145&lt;/SEQUENCE>  &lt;SCREEN_NAME>VENDOR PAYMENT REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200108988\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>145&lt;/SEQ_NO>  &lt;PATH>-117-2145&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106729&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BATCH STATUS REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1100106729&amp;amp;actionId=1100106729&amp;amp;mode=R&amp;amp;actionName=BATCH_STATUS_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_STATUS_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>15&lt;/SEQUENCE>  &lt;SCREEN_NAME>BATCH_STATUS_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1100106729\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>15&lt;/SEQ_NO>  &lt;PATH>-117-215&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108922&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BATCH UPLOAD ISSUES REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108922&amp;amp;actionId=1200108922&amp;amp;mode=R&amp;amp;actionName=BATCH_UPLOAD_ISSUES_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD_ISSUES_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>16&lt;/SEQUENCE>  &lt;SCREEN_NAME>BATCH_UPLOAD_ISSUES_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;=1200108922&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>16&lt;/SEQ_NO>  &lt;PATH>-117-216&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109108&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CREDIT OPERATION REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>17&lt;/SEQUENCE>  &lt;SCREEN_NAME>CREDIT_OPERATION_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>17&lt;/SEQ_NO>  &lt;PATH>-117-217&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109132&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>ANNEXES OL&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109132&amp;amp;actionId=1200109132&amp;amp;mode=R&amp;amp;actionName=ANNEXES_OL&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>ANNEXES_OL&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109132&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109137&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>AUDIT CONFIRMATION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109137&amp;amp;actionId=1200109137&amp;amp;mode=R&amp;amp;actionName=AUDIT_CONFIRMATION&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>AUDIT_CONFIRMATION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109137&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109121&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BOARD OF RESOLUTION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109121&amp;amp;actionId=1200109121&amp;amp;mode=R&amp;amp;actionName=BOARD_OF_RESOLUTION&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>BOARD_OF_RESOLUTION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109121&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109112&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BOARD RESOLUTION CORPORATE GUARANTOR&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109112&amp;amp;actionId=1200109112&amp;amp;mode=R&amp;amp;actionName=BOARD_RESOLUTION_CORPORATE_GUARANTOR&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>BOARD_RESOLUTION_CORPORATE_GUARANTOR&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109112&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109136&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CANCELLATION SALARY DEDUCTION FORM&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109136&amp;amp;actionId=1200109136&amp;amp;mode=R&amp;amp;actionName=CANCELLATION_SALARY_DEDUCTION_FORM&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>CANCELLATION_SALARY_DEDUCTION_FORM&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109136&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109135&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CANCELLATION STANDING ORDER FORM&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109135&amp;amp;actionId=1200109135&amp;amp;mode=R&amp;amp;actionName=CANCELLATION_STANDING_ORDER_FORM&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>CANCELLATION_STANDING_ORDER_FORM&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109135&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109119&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CERTIFICAT DE GAGE SECOND HAND&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109119&amp;amp;actionId=1200109119&amp;amp;mode=R&amp;amp;actionName=CERTIFICAT_DE_GAGE_SECOND_HAND&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>CERTIFICAT_DE_GAGE_SECOND_HAND&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109119&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109113&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CHANGE IN ENGINE NUMBER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109113&amp;amp;actionId=1200109113&amp;amp;mode=R&amp;amp;actionName=CHANGE_IN_ENGINE_NUMBER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>CHANGE_IN_ENGINE_NUMBER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109113&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109114&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CHANGE IN REGISTRATION NUMBER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109114&amp;amp;actionId=1200109114&amp;amp;mode=R&amp;amp;actionName=CHANGE_IN_REGISTRATION_NUMBER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>CHANGE_IN_REGISTRATION_NUMBER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109114&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109115&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CHANGE OF NAME OF HORSEPOWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109115&amp;amp;actionId=1200109115&amp;amp;mode=R&amp;amp;actionName=CHANGE_OF_NAME_OF_HORSEPOWER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>CHANGE_OF_NAME_OF_HORSEPOWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109115&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109116&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CORPORATE GUARANTEE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109116&amp;amp;actionId=1200109116&amp;amp;mode=R&amp;amp;actionName=CORPORATE_GUARANTEE&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>CORPORATE_GUARANTEE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109116&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109155&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>COVER LETTER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109155&amp;amp;actionId=1200109155&amp;amp;mode=R&amp;amp;actionName=COVER_LETTER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>COVER_LETTER&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109155&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109126&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>DIRECT DEBIT FORM&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109126&amp;amp;actionId=1200109126&amp;amp;mode=R&amp;amp;actionName=DIRECT_DEBIT_FORM&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>DIRECT_DEBIT_FORM&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109126&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109117&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>HORSEPOWER FULL&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109117&amp;amp;actionId=1200109117&amp;amp;mode=R&amp;amp;actionName=HORSEPOWER_FULL&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>HORSEPOWER_FULL&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109117&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109143&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE AGREEMENT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109143&amp;amp;actionId=1200109143&amp;amp;mode=R&amp;amp;actionName=LEASE_AGREEMENT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_AGREEMENT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109143&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109131&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE AGREEMENT OL&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109131&amp;amp;actionId=1200109131&amp;amp;mode=R&amp;amp;actionName=LEASE_AGREEMENT_OL&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_AGREEMENT_OL&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109131&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109111&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LETTER OF UNDERTAKING&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109111&amp;amp;actionId=1200109111&amp;amp;mode=R&amp;amp;actionName=LETTER_OF_UNDERTAKING&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>LETTER_OF_UNDERTAKING&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109111&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109118&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LOST HORSEPOWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109118&amp;amp;actionId=1200109118&amp;amp;mode=R&amp;amp;actionName=LOST_HORSEPOWER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>LOST_HORSEPOWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109118&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109120&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>NO LIABILITY LETTER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109120&amp;amp;actionId=1200109120&amp;amp;mode=R&amp;amp;actionName=NO_LIABILITY_LETTER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>NO_LIABILITY_LETTER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109120&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109141&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>OFFER LETTER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109141&amp;amp;actionId=1200109141&amp;amp;mode=R&amp;amp;actionName=OFFER_LETTER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>OFFER_LETTER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109141&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109067&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>REPAYMENT SCHEDULE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109067&amp;amp;actionId=1200109067&amp;amp;mode=R&amp;amp;actionName=REPAYMENT_SCHEDULE&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPAYMENT_SCHEDULE&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>REPAYMENT_SCHEDULE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109067&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109109&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>RIGHT OF SET OFF LETTER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109109&amp;amp;actionId=1200109109&amp;amp;mode=R&amp;amp;actionName=RIGHT_OF_SET_OFF_LETTER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>RIGHT_OF_SET_OFF_LETTER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109109&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109140&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>RV LETTER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109140&amp;amp;actionId=1200109140&amp;amp;mode=R&amp;amp;actionName=RV_LETTER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>RV_LETTER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109140&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109139&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>RV SALES DEED AND CERTIFICATE OF GAGE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109139&amp;amp;actionId=1200109139&amp;amp;mode=R&amp;amp;actionName=RV_SALES_DEED_AND_CERTIFICATE_OF_GAGE&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>RV_SALES_DEED_AND_CERTIFICATE_OF_GAGE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109139&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109134&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>SALARY DEDUCTION FORM&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109134&amp;amp;actionId=1200109134&amp;amp;mode=R&amp;amp;actionName=SALARY_DEDUCTION_FORM&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>SALARY_DEDUCTION_FORM&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109134&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109138&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>SALES DEED&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109138&amp;amp;actionId=1200109138&amp;amp;mode=R&amp;amp;actionName=SALES_DEED&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>SALES_DEED&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109138&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109003&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>SETTLEMENT LETTER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109003&amp;amp;actionId=1200109003&amp;amp;mode=R&amp;amp;actionName=FORECLOSURE_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>FORECLOSURE_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109003&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109127&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>STANDING ORDER FORM&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109127&amp;amp;actionId=1200109127&amp;amp;mode=R&amp;amp;actionName=STANDING_ORDER_FORM&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>STANDING_ORDER_FORM&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109127&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109110&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>SUBORDINATION OF SHAREHOLDERS LOAN&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109110&amp;amp;actionId=1200109110&amp;amp;mode=R&amp;amp;actionName=SUBORDINATION_OF_SHAREHOLDERS_LOAN&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>SUBORDINATION_OF_SHAREHOLDERS_LOAN&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109110&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109142&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>SUPPLIER LETTER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109142&amp;amp;actionId=1200109142&amp;amp;mode=R&amp;amp;actionName=SUPPLIER_LETTER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>SUPPLIER_LETTER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109142&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108947&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BULK INVOICE REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>llmDashboard.do?actionPerformed=getDashboard&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BULK_INVOICE_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>20&lt;/SEQUENCE>  &lt;SCREEN_NAME>BULK_INVOICE_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>llmDashboard.do?actionPerformed=getDashboard&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>20&lt;/SEQ_NO>  &lt;PATH>-117-220&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106855&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>COVERNOTE REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>llmDashboard.do?actionPerformed=getDashboard&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>COVERNOTE_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>37&lt;/SEQUENCE>  &lt;SCREEN_NAME>COVERNOTE_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>llmDashboard.do?actionPerformed=getDashboard&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>37&lt;/SEQ_NO>  &lt;PATH>-117-237&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106539&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>AGEING REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1100106539&amp;amp;actionId=1100106539&amp;amp;mode=R&amp;amp;actionName=AGING_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>AGEING_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>4&lt;/SEQUENCE>  &lt;SCREEN_NAME>AGEING REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1100106539\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>4&lt;/SEQ_NO>  &lt;PATH>-117-24&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108999&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BRANCH WISE SUMMARY&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108999&amp;amp;actionId=1200108999&amp;amp;mode=R&amp;amp;actionName=BRANCH_WISE&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>40&lt;/SEQUENCE>  &lt;SCREEN_NAME>BRANCH WISE SUMMARY&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200108999\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>40&lt;/SEQ_NO>  &lt;PATH>-117-240&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109000&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>USER WISE SUMMARY&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109000&amp;amp;actionId=1200109000&amp;amp;mode=R&amp;amp;actionName=USER_WISE&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>40&lt;/SEQUENCE>  &lt;SCREEN_NAME>USER WISE SUMMARY&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109000\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>40&lt;/SEQ_NO>  &lt;PATH>-117-240&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108980&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER LIMIT REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108980&amp;amp;actionId=1200108980&amp;amp;mode=R&amp;amp;actionName=CUSTOMER_LIMIT_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>CUSTOMER_LIMIT_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>42&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER LIMIT REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200108980\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>42&lt;/SEQ_NO>  &lt;PATH>-117-242&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108996&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>FORECLOSURE REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108996&amp;amp;actionId=1200108996&amp;amp;mode=R&amp;amp;actionName=FORECLOSURE_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>FORECLOSURE_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>57&lt;/SEQUENCE>  &lt;SCREEN_NAME>FORECLOSURE REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200108996\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>57&lt;/SEQ_NO>  &lt;PATH>-117-257&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106710&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>ALCO REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1100106710&amp;amp;actionId=1100106710&amp;amp;mode=R&amp;amp;actionName=ALCO_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>ALCO_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>6&lt;/SEQUENCE>  &lt;SCREEN_NAME>ALCO_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1100106710\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>6&lt;/SEQ_NO>  &lt;PATH>-117-26&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108979&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>GROUP CUSTOMER REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108979&amp;amp;actionId=1200108979&amp;amp;mode=R&amp;amp;actionName=GROUP_CUSTOMER_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>GROUP_CUSTOMER_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>61&lt;/SEQUENCE>  &lt;SCREEN_NAME>GROUP CUSTOMER REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200108979\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>61&lt;/SEQ_NO>  &lt;PATH>-117-261&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108981&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>GROUP LIMIT REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108981&amp;amp;actionId=1200108981&amp;amp;mode=R&amp;amp;actionName=GROUP_LIMIT_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>GROUP_LIMIT_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>62&lt;/SEQUENCE>  &lt;SCREEN_NAME>GROUP LIMIT REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200108981\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>62&lt;/SEQ_NO>  &lt;PATH>-117-262&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108929&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE CAM REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108929&amp;amp;actionId=1200108929&amp;amp;mode=R&amp;amp;actionName=LEASE_CAM_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LEASE_CAM_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>90&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_CAM_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;=1200108929&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>90&lt;/SEQ_NO>  &lt;PATH>-117-290&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108906&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE CREDIT NOTE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108906&amp;amp;actionId=1200108906&amp;amp;mode=R&amp;amp;actionName=LEASE_CREDIT_NOTE&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LEASE_CREDIT_NOTE&lt;/MENU_TYPE>  &lt;SEQUENCE>92&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_CREDIT_NOTE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;=1200108906&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>92&lt;/SEQ_NO>  &lt;PATH>-117-292&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108876&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VAT INVOICE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108876&amp;amp;actionId=1200108876&amp;amp;mode=R&amp;amp;actionName=LEASE_SALE_INVOICE&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>98&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_SALE_INVOICE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200108876\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>98&lt;/SEQ_NO>  &lt;PATH>-117-298&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001024&lt;/ACTION_ID>  &lt;PARENT_ID/>  &lt;DISPLAY_NAME>MASTERS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>MASTER&lt;/MENU_TYPE>  &lt;SEQUENCE>18&lt;/SEQUENCE>  &lt;SCREEN_NAME>GMM&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>18&lt;/SEQ_NO>  &lt;PATH>-118&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001025&lt;/ACTION_ID>  &lt;PARENT_ID>1000001024&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>MASTER&lt;/MENU_TYPE>  &lt;SEQUENCE>105&lt;/SEQUENCE>  &lt;SCREEN_NAME>MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>105&lt;/SEQ_NO>  &lt;PATH>-118-2105&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106525&lt;/ACTION_ID>  &lt;PARENT_ID>1000001025&lt;/PARENT_ID>  &lt;DISPLAY_NAME>SALESMANAGER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>genericMasterAction.do?actionPerformed=displayData&amp;amp;screenId=1100106525&amp;amp;actionId=1100106525&amp;amp;mode=M&amp;amp;actionName=QM_SALESMANAGER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_SALESMANAGER&lt;/MENU_TYPE>  &lt;SEQUENCE>126&lt;/SEQUENCE>  &lt;SCREEN_NAME>SALESMANAGER (MAKER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>genericMasterAction.do?actionPerformed=displaydata&amp;amp;masterId=1100106525&amp;amp;mode=M&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>126&lt;/SEQ_NO>  &lt;PATH>-118-2105-3126&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001026&lt;/ACTION_ID>  &lt;PARENT_ID>1000001024&lt;/PARENT_ID>  &lt;DISPLAY_NAME>AUTHOR&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>MASTER&lt;/MENU_TYPE>  &lt;SEQUENCE>305&lt;/SEQUENCE>  &lt;SCREEN_NAME>AUTHOR&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>305&lt;/SEQ_NO>  &lt;PATH>-118-2305&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001300&lt;/ACTION_ID>  &lt;PARENT_ID/>  &lt;DISPLAY_NAME>BATCH UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>313&lt;/SEQUENCE>  &lt;SCREEN_NAME>BATCH_UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>313&lt;/SEQ_NO>  &lt;PATH>-1313&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001301&lt;/ACTION_ID>  &lt;PARENT_ID>1000001300&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>314&lt;/SEQUENCE>  &lt;SCREEN_NAME>BATCH UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>314&lt;/SEQ_NO>  &lt;PATH>-1313-2314&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>9200109154&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>AP INT GL MAP CONFIG&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=9200109154&amp;amp;actionId=9200109154&amp;amp;mode=M&amp;amp;moduleId=AP_INT_GL_MAP_CONFIG&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>AP INT GL MAP CONFIG&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106475&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BULK RECEIPT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106475&amp;amp;actionId=1100106475&amp;amp;mode=M&amp;amp;moduleId=OTC_IMD&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>BULK RECEIPT&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108861&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>INSTRUMENT MANAGEMENT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108861&amp;amp;actionId=1200108861&amp;amp;mode=M&amp;amp;moduleId=1200000300&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>INSTR MANAGEMENT MAKER/AUTHOR&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109167&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE REGISTRATION TRACKING&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109167&amp;amp;actionId=1200109167&amp;amp;mode=M&amp;amp;moduleId=LEASE_REG_DTL&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE REGISTRATION TRACKING&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106491&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>NEGATIVE LIST&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>NEGATIVE LIST UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109156&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>OFFLINE INSURANCE DETAILS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109156&amp;amp;actionId=1200109156&amp;amp;mode=M&amp;amp;moduleId=OFFLINE_INS_DTLS&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>OFFLINE INSURANCE DETAILS&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106494&lt;/ACTION_ID>  &lt;PARENT_ID>1100106491&lt;/PARENT_ID>  &lt;DISPLAY_NAME>SDN&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106494&amp;amp;actionId=1100106494&amp;amp;mode=M&amp;amp;moduleId=SDN&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>SDN NEGATIVE LIST UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315-41&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106496&lt;/ACTION_ID>  &lt;PARENT_ID>1100106491&lt;/PARENT_ID>  &lt;DISPLAY_NAME>TALIBAN&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106496&amp;amp;actionId=1100106496&amp;amp;mode=M&amp;amp;moduleId=TALIBAN&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>2&lt;/SEQUENCE>  &lt;SCREEN_NAME>TALIBAN NEGATIVE LIST UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>2&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315-42&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106493&lt;/ACTION_ID>  &lt;PARENT_ID>1100106491&lt;/PARENT_ID>  &lt;DISPLAY_NAME>AL-QAIDA&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106493&amp;amp;actionId=1100106493&amp;amp;mode=M&amp;amp;moduleId=ALQAIDA&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>AL-QAIDA NEGATIVE LIST UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315-43&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109123&lt;/ACTION_ID>  &lt;PARENT_ID>1100106491&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MCIB&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109123&amp;amp;actionId=1200109123&amp;amp;mode=M&amp;amp;moduleId=MCIB&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>MCIB&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315-43&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109105&lt;/ACTION_ID>  &lt;PARENT_ID>1100106491&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CAUTION LIST&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109105&amp;amp;actionId=1200109105&amp;amp;mode=M&amp;amp;moduleId=CAUTION_LIST&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>CAUTION LIST&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315-4315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100010120&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PDC/ECS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100010120&amp;amp;actionId=1100010120&amp;amp;mode=M&amp;amp;moduleId=PDC&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PDC/ECS&lt;/MENU_TYPE>  &lt;SEQUENCE>316&lt;/SEQUENCE>  &lt;SCREEN_NAME>PDC/ECS&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>316&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3316&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106572&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>QT FLEET DETAIL UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106572&amp;amp;actionId=1100106572&amp;amp;mode=M&amp;amp;moduleId=BH00000195&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QT_FLEET_DETAIL&lt;/MENU_TYPE>  &lt;SEQUENCE>320&lt;/SEQUENCE>  &lt;SCREEN_NAME>QT_FLEET_DETAIL UPLOAD  (AUTOAUTH)&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>320&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3320&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106753&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE ASSET PRICE MST UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106753&amp;amp;actionId=1200106753&amp;amp;mode=M&amp;amp;moduleId=1200000199&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_ASSET_PRICE_MST&lt;/MENU_TYPE>  &lt;SEQUENCE>321&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_ASSET_PRICE_MST UPLOAD  (MAKER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>321&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3321&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106841&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE ASSET VENDOR UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106841&amp;amp;actionId=1200106841&amp;amp;mode=M&amp;amp;moduleId=1200000209&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_ASSET_VENDOR&lt;/MENU_TYPE>  &lt;SEQUENCE>321&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_ASSET_VENDOR  (MAKER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>321&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3321&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106844&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE VENDOR GSTIN UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106844&amp;amp;actionId=1200106844&amp;amp;mode=M&amp;amp;moduleId=1200000210&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_VENDOR_GSTIN&lt;/MENU_TYPE>  &lt;SEQUENCE>321&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_VENDOR_GSTIN  (MAKER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>321&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3321&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106730&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MANUAL VOUCHER UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106730&amp;amp;actionId=1100106730&amp;amp;mode=M&amp;amp;moduleId=BH00000196&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>321&lt;/SEQUENCE>  &lt;SCREEN_NAME>MANUAL VOUCHER UPLOAD (AUTOAUTH)&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>321&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3321&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108938&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER INVOICE CONFIG UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108938&amp;amp;actionId=1200108938&amp;amp;mode=M&amp;amp;moduleId=1200000213&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_CUST_INVOICE_CONFIG_DTLS&lt;/MENU_TYPE>  &lt;SEQUENCE>322&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER INVOICE CONFIG UPLOAD (MAKER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>322&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3322&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106834&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE BATTERY PRICE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106834&amp;amp;actionId=1200106834&amp;amp;mode=M&amp;amp;moduleId=1200000207&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_BATTERY_PRICE&lt;/MENU_TYPE>  &lt;SEQUENCE>322&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_BATTERY_PRICE UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>322&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3322&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106763&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE TYRE PRICE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106763&amp;amp;actionId=1200106763&amp;amp;mode=M&amp;amp;moduleId=1200000202&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_TYRE_PRICE&lt;/MENU_TYPE>  &lt;SEQUENCE>322&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_TYRE_PRICE UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>322&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3322&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108934&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MANUAL ADVICE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108934&amp;amp;actionId=1200108934&amp;amp;mode=M&amp;amp;moduleId=1200000212&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QT_MANUALADVICE&lt;/MENU_TYPE>  &lt;SEQUENCE>322&lt;/SEQUENCE>  &lt;SCREEN_NAME>MANUAL ADVICE UPLOAD (MAKER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>322&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3322&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108997&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE ASSET CESSMAPPING UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108997&amp;amp;actionId=1200108997&amp;amp;mode=M&amp;amp;moduleId=1200000303&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_ASSET_CESSMAPPING&lt;/MENU_TYPE>  &lt;SEQUENCE>323&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE ASSET CESSMAPPING UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>323&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3323&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106771&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE ASSET VARIANT UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106771&amp;amp;actionId=1200106771&amp;amp;mode=M&amp;amp;moduleId=1200000205&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_ASSET_VARIANT&lt;/MENU_TYPE>  &lt;SEQUENCE>323&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_ASSET_VARIANT UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>323&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3323&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108948&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE FC AMNT FORMULA UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108948&amp;amp;actionId=1200108948&amp;amp;mode=M&amp;amp;moduleId=1200000301&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_FC_AMNT_FORMULA&lt;/MENU_TYPE>  &lt;SEQUENCE>323&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_FC_AMNT_FORMULA UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>323&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3323&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108965&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE SERVICE CHRG AMT UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108965&amp;amp;actionId=1200108965&amp;amp;mode=M&amp;amp;moduleId=1200000302&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_SERVICE_CHRG_AMT&lt;/MENU_TYPE>  &lt;SEQUENCE>323&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE SERVICE CHRG AMT UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>323&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3323&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109004&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE VENDOR BANK UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109004&amp;amp;actionId=1200109004&amp;amp;mode=M&amp;amp;moduleId=1200000304&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_VENDOR_BANK_MASTER&lt;/MENU_TYPE>  &lt;SEQUENCE>323&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE VENDOR BANK UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>323&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3323&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109006&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE VENDOR MAPPING UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109006&amp;amp;actionId=1200109006&amp;amp;mode=M&amp;amp;moduleId=1200000305&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_VENDOR_MAPPING&lt;/MENU_TYPE>  &lt;SEQUENCE>323&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE VENDOR MAPPING UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>323&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3323&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106831&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE MAINT SLABS UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106831&amp;amp;actionId=1200106831&amp;amp;mode=M&amp;amp;moduleId=1200000206&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_MAINT_SLABS&lt;/MENU_TYPE>  &lt;SEQUENCE>324&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_MAINT_SLABS UPLOAD &lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>324&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3324&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109034&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BULK CLOSURE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109034&amp;amp;actionId=1200109034&amp;amp;mode=M&amp;amp;moduleId=BULK_CLOSURE&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>350&lt;/SEQUENCE>  &lt;SCREEN_NAME>BULK CLOSURE MAKER/AUTHOR&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>350&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3350&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106724&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CHARGE X GST UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106724&amp;amp;actionId=1100106724&amp;amp;mode=M&amp;amp;moduleId=BH00000023&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_CHARGE_X_GST&lt;/MENU_TYPE>  &lt;SEQUENCE>353&lt;/SEQUENCE>  &lt;SCREEN_NAME>CHARGE_X_GST UPLOAD  (MAKER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>353&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3353&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106838&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE RESIDUAL VALUE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106838&amp;amp;actionId=1200106838&amp;amp;mode=M&amp;amp;moduleId=1200000208&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_RESIDUAL_VALUE&lt;/MENU_TYPE>  &lt;SEQUENCE>996&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_RESIDUAL_VALUE UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>996&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3996&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001302&lt;/ACTION_ID>  &lt;PARENT_ID>1000001300&lt;/PARENT_ID>  &lt;DISPLAY_NAME>AUTHOR&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>320&lt;/SEQUENCE>  &lt;SCREEN_NAME>BATCH UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>320&lt;/SEQ_NO>  &lt;PATH>-1313-2320&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>9200109155&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>AP INT GL MAP CONFIG&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=9200109155&amp;amp;actionId=9200109155&amp;amp;mode=A&amp;amp;moduleId=AP_INT_GL_MAP_CONFIG&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>AP INT GL MAP CONFIG&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109036&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BULK CLOSURE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109036&amp;amp;actionId=1200109036&amp;amp;mode=A&amp;amp;moduleId=BULK_CLOSURE&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>BULK CLOSURE MAKER/AUTHOR&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109045&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BULK RECEIPT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109045&amp;amp;actionId=1200109045&amp;amp;mode=A&amp;amp;moduleId=OTC_IMD&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>BULK RECEIPT&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108863&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER RECEIPT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108863&amp;amp;actionId=1200108863&amp;amp;mode=A&amp;amp;moduleId=CUSTOMER_RECEIPT&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER RECEIPT MAKER/AUTHOR&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106754&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE ASSET PRICE MST UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106754&amp;amp;actionId=1200106754&amp;amp;mode=A&amp;amp;moduleId=1200000199&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_ASSET_PRICE_MST&lt;/MENU_TYPE>  &lt;SEQUENCE>322&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_ASSET_PRICE_MST UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>322&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3322&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100010121&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PDC/ECS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100010121&amp;amp;actionId=1100010121&amp;amp;mode=A&amp;amp;moduleId=PDC&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PDC/ECS&lt;/MENU_TYPE>  &lt;SEQUENCE>322&lt;/SEQUENCE>  &lt;SCREEN_NAME>PDC/ECS&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>322&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3322&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>9200109157&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE VENDOR BANK UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=9200109157&amp;amp;actionId=9200109157&amp;amp;mode=A&amp;amp;moduleId=1200000304&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_VENDOR_BANK_MASTER&lt;/MENU_TYPE>  &lt;SEQUENCE>323&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE VENDOR BANK UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>323&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3323&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108935&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MANUAL ADVICE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108935&amp;amp;actionId=1200108935&amp;amp;mode=A&amp;amp;moduleId=1200000212&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QT_MANUALADVICE&lt;/MENU_TYPE>  &lt;SEQUENCE>323&lt;/SEQUENCE>  &lt;SCREEN_NAME>MANUAL ADVICE UPLOAD (AUTHOR)&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>323&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3323&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106772&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE ASSET VARIANT UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106772&amp;amp;actionId=1200106772&amp;amp;mode=A&amp;amp;moduleId=1200000205&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_ASSET_VARIANT&lt;/MENU_TYPE>  &lt;SEQUENCE>324&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_ASSET_VARIANT UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>324&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3324&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108949&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE FC AMNT FORMULA UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108949&amp;amp;actionId=1200108949&amp;amp;mode=A&amp;amp;moduleId=1200000301&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_FC_AMNT_FORMULA&lt;/MENU_TYPE>  &lt;SEQUENCE>324&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_FC_AMNT_FORMULA UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>324&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3324&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108966&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE SERVICE CHRG AMT UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108966&amp;amp;actionId=1200108966&amp;amp;mode=A&amp;amp;moduleId=1200000302&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_SERVICE_CHRG_AMT&lt;/MENU_TYPE>  &lt;SEQUENCE>324&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE SERVICE CHRG AMT UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>324&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3324&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106842&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE ASSET VENDOR UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106842&amp;amp;actionId=1200106842&amp;amp;mode=A&amp;amp;moduleId=1200000209&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_ASSET_VENDOR&lt;/MENU_TYPE>  &lt;SEQUENCE>326&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_ASSET_VENDOR  (AUTHOR)&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>326&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3326&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106845&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE VENDOR GSTIN UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106845&amp;amp;actionId=1200106845&amp;amp;mode=A&amp;amp;moduleId=1200000210&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_VENDOR_GSTIN&lt;/MENU_TYPE>  &lt;SEQUENCE>326&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_VENDOR_GSTIN  (AUTHOR)&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>326&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3326&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106732&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MANUAL VOUCHER UPLOAD AUTHOR&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106732&amp;amp;actionId=1100106732&amp;amp;mode=A&amp;amp;moduleId=BH00000196&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>326&lt;/SEQUENCE>  &lt;SCREEN_NAME>MANUAL VOUCHER UPLOAD AUTHOR&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>326&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3326&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106835&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE BATTERY PRICE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106835&amp;amp;actionId=1200106835&amp;amp;mode=A&amp;amp;moduleId=1200000207&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_BATTERY_PRICE&lt;/MENU_TYPE>  &lt;SEQUENCE>327&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_BATTERY_PRICE UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>327&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3327&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106764&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE TYRE PRICE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106764&amp;amp;actionId=1200106764&amp;amp;mode=A&amp;amp;moduleId=1200000202&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_TYRE_PRICE&lt;/MENU_TYPE>  &lt;SEQUENCE>327&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_TYRE_PRICE UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>327&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3327&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106832&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE MAINT SLABS UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>BATCHUPLOADACTION.DO?ACTIONPERFORMED=BATCHUPLOAD&amp;amp;SCREENID=1200106832&amp;amp;ACTIONID=1200106832&amp;amp;MODE=A&amp;amp;MODULEID=1200000206&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_MAINT_SLABS&lt;/MENU_TYPE>  &lt;SEQUENCE>329&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_MAINT_SLABS UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>329&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3329&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109104&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>NEGATIVE LIST&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>332&lt;/SEQUENCE>  &lt;SCREEN_NAME>NEGATIVE LIST VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>332&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3332&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109106&lt;/ACTION_ID>  &lt;PARENT_ID>1200109104&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CAUTION LIST&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109106&amp;amp;actionId=1200109106&amp;amp;mode=A&amp;amp;moduleId=CAUTION_LIST&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>CAUTION LIST&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3332-4315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106725&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CHARGE X GST UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106725&amp;amp;actionId=1100106725&amp;amp;mode=A&amp;amp;moduleId=BH00000023&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_CHARGE_X_GST&lt;/MENU_TYPE>  &lt;SEQUENCE>351&lt;/SEQUENCE>  &lt;SCREEN_NAME>CHARGE_X_GST UPLOAD  (AUTHOR)&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>351&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3351&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106839&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE RESIDUAL VALUE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106839&amp;amp;actionId=1200106839&amp;amp;mode=A&amp;amp;moduleId=1200000208&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_RESIDUAL_VALUE&lt;/MENU_TYPE>  &lt;SEQUENCE>997&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_RESIDUAL_VALUE UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>997&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3997&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001303&lt;/ACTION_ID>  &lt;PARENT_ID>1000001300&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>330&lt;/SEQUENCE>  &lt;SCREEN_NAME>BATCH UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>330&lt;/SEQ_NO>  &lt;PATH>-1313-2330&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>9200109156&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>AP INT GL MAP CONFIG&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=9200109156&amp;amp;actionId=9200109156&amp;amp;mode=V&amp;amp;moduleId=AP_INT_GL_MAP_CONFIG&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>AP INT GL MAP CONFIG&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109168&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE REGISTRATION TRACKING&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109168&amp;amp;actionId=1200109168&amp;amp;mode=V&amp;amp;moduleId=LEASE_REG_DTL&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>316&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE REGISTRATION TRACKING&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>316&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3316&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109157&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>OFFLINE INSURANCE DETAILS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109157&amp;amp;actionId=1200109157&amp;amp;mode=V&amp;amp;moduleId=OFFLINE_INS_DTLS&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>316&lt;/SEQUENCE>  &lt;SCREEN_NAME>OFFLINE INSURANCE DETAILS&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>316&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3316&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106755&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE ASSET PRICE MST UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106755&amp;amp;actionId=1200106755&amp;amp;mode=V&amp;amp;moduleId=1200000199&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_ASSET_PRICE_MST&lt;/MENU_TYPE>  &lt;SEQUENCE>321&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_ASSET_PRICE_MST UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>321&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3321&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106731&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MANUAL VOUCHER UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106731&amp;amp;actionId=1100106731&amp;amp;mode=V&amp;amp;moduleId=BH00000196&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>322&lt;/SEQUENCE>  &lt;SCREEN_NAME>MANUAL VOUCHER UPLOAD (VIEWER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>322&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3322&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108940&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER INVOICE CONFIG UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108940&amp;amp;actionId=1200108940&amp;amp;mode=V&amp;amp;moduleId=1200000213&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_CUST_INVOICE_CONFIG_DTLS&lt;/MENU_TYPE>  &lt;SEQUENCE>324&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER INVOICE CONFIG UPLOAD (VIEWER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>324&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3324&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108936&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MANUAL ADVICE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108936&amp;amp;actionId=1200108936&amp;amp;mode=V&amp;amp;moduleId=1200000212&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QT_MANUALADVICE&lt;/MENU_TYPE>  &lt;SEQUENCE>324&lt;/SEQUENCE>  &lt;SCREEN_NAME>MANUAL ADVICE UPLOAD (VIEWER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>324&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3324&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106773&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE ASSET VARIANT UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106773&amp;amp;actionId=1200106773&amp;amp;mode=V&amp;amp;moduleId=1200000205&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_ASSET_VARIANT&lt;/MENU_TYPE>  &lt;SEQUENCE>325&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_ASSET_VARIANT UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>325&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3325&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108950&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE FC AMNT FORMULA UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108950&amp;amp;actionId=1200108950&amp;amp;mode=V&amp;amp;moduleId=1200000301&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_FC_AMNT_FORMULA&lt;/MENU_TYPE>  &lt;SEQUENCE>325&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_FC_AMNT_FORMULA UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>325&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3325&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108967&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE SERVICE CHRG AMT UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108967&amp;amp;actionId=1200108967&amp;amp;mode=V&amp;amp;moduleId=1200000302&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_SERVICE_CHRG_AMT&lt;/MENU_TYPE>  &lt;SEQUENCE>325&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE SERVICE CHRG AMT UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>325&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3325&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109005&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE VENDOR BANK UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109005&amp;amp;actionId=1200109005&amp;amp;mode=V&amp;amp;moduleId=1200000304&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_VENDOR_BANK_MASTER&lt;/MENU_TYPE>  &lt;SEQUENCE>325&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE VENDOR BANK UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>325&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3325&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109007&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE VENDOR MAPPING UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109007&amp;amp;actionId=1200109007&amp;amp;mode=V&amp;amp;moduleId=1200000305&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_VENDOR_MAPPING&lt;/MENU_TYPE>  &lt;SEQUENCE>325&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE VENDOR MAPPING UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>325&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3325&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106476&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BULK RECEIPT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106476&amp;amp;actionId=1100106476&amp;amp;mode=V&amp;amp;moduleId=OTC_IMD&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>332&lt;/SEQUENCE>  &lt;SCREEN_NAME>BULK RECEIPT&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>332&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3332&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108866&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER RECEIPT VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108866&amp;amp;actionId=1200108866&amp;amp;mode=V&amp;amp;moduleId=CUSTOMER_RECEIPT&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>332&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER RECEIPT VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>332&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3332&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108862&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>INSTRUMENT MANAGEMENT VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108861&amp;amp;actionId=1200108861&amp;amp;mode=V&amp;amp;moduleId=1200000300&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>332&lt;/SEQUENCE>  &lt;SCREEN_NAME>INSTR MANAGEMENT VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>332&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3332&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106490&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>NEGATIVE LIST&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>332&lt;/SEQUENCE>  &lt;SCREEN_NAME>NEGATIVE LIST VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>332&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3332&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106484&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>RESCH VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106484&amp;amp;actionId=1100106484&amp;amp;mode=V&amp;amp;moduleId=RESCH&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>332&lt;/SEQUENCE>  &lt;SCREEN_NAME>RESCH&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>332&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3332&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106495&lt;/ACTION_ID>  &lt;PARENT_ID>1100106490&lt;/PARENT_ID>  &lt;DISPLAY_NAME>SDN&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106495&amp;amp;actionId=1100106495&amp;amp;mode=V&amp;amp;moduleId=SDN&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>SDN NEGATIVE LIST&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3332-41&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106497&lt;/ACTION_ID>  &lt;PARENT_ID>1100106490&lt;/PARENT_ID>  &lt;DISPLAY_NAME>TALIBAN&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106497&amp;amp;actionId=1100106497&amp;amp;mode=V&amp;amp;moduleId=TALIBAN&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>2&lt;/SEQUENCE>  &lt;SCREEN_NAME>TALIBAN NEGATIVE LIST&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>2&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3332-42&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106492&lt;/ACTION_ID>  &lt;PARENT_ID>1100106490&lt;/PARENT_ID>  &lt;DISPLAY_NAME>AL-QAIDA&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106492&amp;amp;actionId=1100106492&amp;amp;mode=V&amp;amp;moduleId=ALQAIDA&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>AL-QAIDA NEGATIVE LIST&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3332-43&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109122&lt;/ACTION_ID>  &lt;PARENT_ID>1100106490&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MCIB&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109122&amp;amp;actionId=1200109122&amp;amp;mode=V&amp;amp;moduleId=MCIB&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>MCIB&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3332-43&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109107&lt;/ACTION_ID>  &lt;PARENT_ID>1100106490&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CAUTION LIST&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109107&amp;amp;actionId=1200109107&amp;amp;mode=V&amp;amp;moduleId=CAUTION_LIST&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>CAUTION LIST&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3332-4315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109035&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BULK CLOSURE VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109035&amp;amp;actionId=1200109035&amp;amp;mode=V&amp;amp;moduleId=BULK_CLOSURE&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>333&lt;/SEQUENCE>  &lt;SCREEN_NAME>BULK CLOSURE&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>333&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3333&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100010128&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PDC/ECS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100010128&amp;amp;actionId=1100010128&amp;amp;mode=V&amp;amp;moduleId=PDC&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PDC/ECS&lt;/MENU_TYPE>  &lt;SEQUENCE>335&lt;/SEQUENCE>  &lt;SCREEN_NAME>PDC/ECS &lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>335&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3335&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106843&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE ASSET VENDOR UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106843&amp;amp;actionId=1200106843&amp;amp;mode=V&amp;amp;moduleId=1200000209&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_ASSET_VENDOR&lt;/MENU_TYPE>  &lt;SEQUENCE>338&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_ASSET_VENDOR  (VIEWER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>338&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3338&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106846&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE VENDOR GSTIN UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106846&amp;amp;actionId=1200106846&amp;amp;mode=V&amp;amp;moduleId=1200000210&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_VENDOR_GSTIN&lt;/MENU_TYPE>  &lt;SEQUENCE>338&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_VENDOR_GSTIN  (VIEWER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>338&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3338&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106836&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE BATTERY PRICE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106836&amp;amp;actionId=1200106836&amp;amp;mode=V&amp;amp;moduleId=1200000207&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_BATTERY_PRICE&lt;/MENU_TYPE>  &lt;SEQUENCE>339&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_BATTERY_PRICE UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>339&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3339&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106765&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE TYRE PRICE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106765&amp;amp;actionId=1200106765&amp;amp;mode=V&amp;amp;moduleId=1200000202&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_TYRE_PRICE&lt;/MENU_TYPE>  &lt;SEQUENCE>339&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_TYRE_PRICE UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>339&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3339&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106833&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE MAINT SLABS UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>BATCHUPLOADACTION.DO?ACTIONPERFORMED=BATCHUPLOAD&amp;amp;SCREENID=1200106833&amp;amp;ACTIONID=1200106833&amp;amp;MODE=V&amp;amp;MODULEID=1200000206&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_MAINT_SLABS&lt;/MENU_TYPE>  &lt;SEQUENCE>341&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_MAINT_SLABS UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>341&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3341&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106726&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CHARGE X GST UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106726&amp;amp;actionId=1100106726&amp;amp;mode=V&amp;amp;moduleId=BH00000023&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_CHARGE_X_GST&lt;/MENU_TYPE>  &lt;SEQUENCE>352&lt;/SEQUENCE>  &lt;SCREEN_NAME>CHARGE_X_GST UPLOAD  (VIEWER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>352&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3352&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106840&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE RESIDUAL VALUE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106840&amp;amp;actionId=1200106840&amp;amp;mode=V&amp;amp;moduleId=1200000208&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_RESIDUAL_VALUE&lt;/MENU_TYPE>  &lt;SEQUENCE>998&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_RESIDUAL_VALUE UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>998&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3998&lt;/PATH> &lt;/ROW>&lt;/ROWSET>&quot;;
		           // alert(menuDataXml)
					setMenuData(menuDataXml);
					renderItemsInto(&quot;verticalMenu&quot;);
		
		
					
				
			
		
	

					

	
		

		
		
			




  
 
    
  miFIN
    
	
	
	    
	
	
	

  
  
  
  
	
  
   	  
			 		 
						  
						  
						  
					  
		
 
 




		
	
	      	
   	
   		
		
		
		
		
			
							CHARGE DETAILS
							
								
									 
									
									
									
			
		
		
				
				
		 
         
         
                  
         
         
         
          
          
          
          
          
          
          
             
        
         
         
		
				
				
								
				
		
		
			
		      	
					
						
							
					 
						 Charge Type
						 Dr/Cr Note
						 Misc. Bill
						
						 Invoice No
						 Charge Id
						 Charge Amount
						 Charge/Invoice Date
						Bill Start Date
						Bill End Date
						 Payment Due Date
						
						 Maker Remarks  
				        
				        
							
				        
				        
						
					    	 Send To Author  
						
						
					
				
			
			
				
				SelectReceivablePayable SelectPRO RATA INTEREST        
			
		
		         
	     
	     
	     
	     
	
	              




	
		
		Qualtech Consultants Pvt. Ltd.
		
		
		VERSION 1.0.1.295
		
			
			   
				 
		
	
		
 


	var objText = null;			
	                  
	function refreshCustomerServiceInLos()
	{
	  var pageURL = $(location).attr(&quot;href&quot;);
       var pathName = pageURL.substring(pageURL.lastIndexOf(&quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot;) + 1,Number(pageURL.length));
       $(location).attr(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;, pathName);
	 } 

$(window).on(&quot; , &quot;'&quot; , &quot;load&quot; , &quot;'&quot; , &quot;, function(){
	if(!$(&quot;div&quot;).hasClass(&quot;menu_tab&quot;)){
	
		$(&quot;body:not(.body_searchlist)&quot;).addClass(&quot;menuHavingBody&quot;);
		//$(&quot;body&quot;).not(&quot;.body_searchlist&quot;).addClass(&quot;menuHavingBody&quot;);
		//$(&quot;body&quot;).addClass(&quot;menuHavingBody&quot;);
	}
	if($(&quot;.subheaderSection div div:nth-of-type(2)&quot;).text()!==&quot;&quot;){
	$(&quot;.slimScrollDiv&quot;).addClass(&quot;responsiveMenu&quot;);
	}	
	$(&quot;#securityCheckList&quot;).next(&quot;table&quot;).find(&quot;td:nth-of-type(1)&quot;).css(&quot;width&quot;,&quot;5%&quot;);
	$(&quot;#secContainer #securityCheckList tr:not(&quot; , &quot;'&quot; , &quot;:nth-child(1)&quot; , &quot;'&quot; , &quot;) td:nth-child(9)&quot;).css(&quot;text-align&quot;,&quot;right&quot;);
	if($(document).height()>($(window).height())){
	$(&quot;.text_footer&quot;).addClass(&quot;posrell&quot;);
	}
	$(&quot;a,.blueBotton&quot;).click(function(){
	if($(document).height()>($(window).height())){$(&quot;.text_footer&quot;).addClass(&quot;posrell&quot;);}
	});

});


function disableAllElementsAjax()
		    {
		    	
		  	   
	                  for(count=0; count &lt; document.forms[0].elements.length; count+=1)
				        {
				        	theelement = document.forms[0].elements[count];
				            if(theelement.name != null &amp;&amp; theelement.name != &quot;btn_one&quot; &amp;&amp;theelement.name !=&quot;CreditOfficerHistory&quot; &amp;&amp;theelement.name != &quot;btnCam&quot; &amp;&amp;theelement.name != &quot;btnSanction&quot;)
				            {
				            	theelement.disabled = true;
				            }
				        }
				      
			  
		    }           

$(window).on(&quot; , &quot;'&quot; , &quot;load&quot; , &quot;'&quot; , &quot;, function(){
	if($(&quot;.subheaderSection div div:nth-of-type(2)&quot;).text()!==&quot;&quot;){
		$(&quot;.slimScrollDiv&quot;).addClass(&quot;responsiveMenu&quot;);
	}	
	$(&quot;.menu_tab&quot;).parents(&quot;body&quot;).addClass(&quot;menuHavingBody&quot;);
	//1.0.0.1 start
	if(!$(&quot;textarea&quot;).prop(&quot;disabled&quot;))
	{
			$(&quot;textarea&quot;).wrap(&quot; , &quot;'&quot; , &quot;&lt;span class=&quot;textarea-span&quot; style=&quot;position: relative;float: left;width: 100%;&quot;>&lt;/span>&quot; , &quot;'&quot; , &quot;);
			$(&quot;.textarea-span&quot;).append(&quot; , &quot;'&quot; , &quot;&lt;span class=&quot;edit-textarea&quot;>&lt;/span>&quot; , &quot;'&quot; , &quot;);
	
	} // 1.0.0.1 end	
	$(document).on(&quot;click&quot;,&quot;.edit-textarea&quot;, function(){
			objText = $(this);
			//Start Added by 1.0.0.2
			if(objText.prev(&quot;textarea&quot;).prop(&quot;disabled&quot;))
				return;
			//End Added by 1.0.0.2
			$(&quot;.justbeforeform&quot;).remove();
			$(&quot;.textarea-popup-content&quot;).remove();
			var eachtareacontent = $(this).prev(&quot;textarea&quot;).val();
			$(&quot;body&quot;).append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;justbeforeform&quot;>&lt;/div> &lt;div class=&quot;textarea-popup-content&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;);
			$(&quot;.textarea-popup-content&quot;).wrapInner(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;div_popup&quot; contenteditable=&quot;true&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;);
			$(&quot;.textarea-popup-content&quot;).animate({top: &quot;100px&quot;});
			
			$(&quot;.div_popup&quot;).text(eachtareacontent);
			$(&quot;.div_popup&quot;).after(&quot; , &quot;'&quot; , &quot;&lt;input type=&quot;button&quot; class=&quot;ok_btn&quot; value=&quot;Close&quot; style=&quot;float:right&quot; />&quot; , &quot;'&quot; , &quot;);
			$(&quot; , &quot;'&quot; , &quot;html, body&quot; , &quot;'&quot; , &quot;).animate({scrollTop : 0},800);
		
			$(document).on(&quot;click&quot;,&quot;.ok_btn&quot;,function(){
			var tyu=$(&quot;.div_popup&quot;).text();
		
				objText.prev(&quot;textarea&quot;).val(tyu);
				$(&quot;.ok_btn&quot;).parent(&quot;.textarea-popup-content&quot;).siblings(&quot;.justbeforeform&quot;).fadeOut(0);
				$(&quot;.ok_btn&quot;).parent(&quot;.textarea-popup-content&quot;).animate({top: &quot;-600px&quot;});
				$(&quot;.ok_btn&quot;).parent(&quot;.textarea-popup-content&quot;).siblings(&quot;.justbeforeform&quot;).css(&quot;display&quot;,&quot;none&quot;);
				//$(&quot;.ok_btn&quot;).parent(&quot;.textarea-popup-content&quot;).css(&quot;display&quot;,&quot;none&quot;);
			});
		});
});

       

	



  

/html[1]/body[@class=&quot;menuHavingBody&quot;]&quot;) or . = concat(&quot;
	
			


 
 
 
	
	
	
	
	
	
	 







//Code added by Naveen Baghel::10/12/2015


setTimeout(&quot;setToolTip();&quot;,100);

function setTitleforVarFields()
{
	for(count=0; count &lt; document.forms[0].elements.length; count+=1)
    {
    	theelement = document.forms[0].elements[count];
    	if(theelement)
        {
	        if(theelement.type == &quot;text&quot;)
	        {
	        	theelement.title = (theelement.value).toUpperCase();
	        	
	        }
	      
	        
	     }
    }   
}


   function setToolTip()

   {try{

   if(document.forms[0])

   {

    for(count=0; count &lt; document.forms[0].elements.length; count+=1)

        {

          theelement = document.forms[0].elements[count];

                   if(theelement.options != null)

                   {
                     showTooltip(theelement);
                   }

                   else

                   {

                    if(theelement.type!=&quot;checkbox&quot; &amp;&amp; theelement.type!=&quot;radio&quot; &amp;&amp;  theelement.value!=null &amp;&amp; theelement.value!=&quot;&quot; )

                     theelement.title=theelement.value;  
                     changeToolTipText(theelement);     
                   }

        }
    }

    }catch(err){

    //alert(err);

    }

    }
    
    function setToolTipText(theelement)
    {
       theelement.title=theelement.value;   
    }

function showTooltip(theelement)

                        {    
                                                                                             

            for(i=0; i&lt;theelement.options.length; i++)

               {

                 theelement.options[i].title=theelement.options[i].text;

               }

               if(theelement.addEventListener){

                                                  theelement.addEventListener(&quot;change&quot;, function(e){

                                                    e = e || event;

                                                    changeToolTip();

                                                  }, false);
                                                 

                                                }

                                                else if(theelement.attachEvent){

                                                  theelement.attachEvent(&quot;onchange&quot;, function(e){

                                                    e = e || event;

                                                    changeToolTip();

                                                  });
                                                  
                                                }

                                                try

                                                {

                if(parseInt(theelement.options.length)!=0)

               theelement.title=theelement.options[theelement.selectedIndex].text;

               else

               theelement.title=theelement.value;

               }

               catch(err)

               {

                        theelement.title=&quot;&quot;;

               }

                        }

        
   function changeToolTipText(theelement)
   {
       if(theelement.addEventListener){

                                                  theelement.addEventListener(&quot;keyup&quot;, function(e){

                                                    e = e || event;

                                                    setToolTipText(theelement);

                                                  }, false);
                                                 

                                                }

                                                else if(theelement.attachEvent){

                                                  theelement.attachEvent(&quot;onkeyup&quot;, function(e){

                                                    e = e || event;

                                                    setToolTipText(theelement);

                                                  });
                                                  
                                                }
   }
        

    function changeToolTip()

    {try{

     for(count=0; count &lt; document.forms[0].elements.length; count+=1)

       {

         theelement = document.forms[0].elements[count];

           if(theelement.options != null)

             {

                if(parseInt(theelement.options.length)!=0)

                 theelement.title=theelement.options[theelement.selectedIndex].text;

                 else

                 theelement.title=theelement.value;

             }
        }}

        catch(err){
       

        }

    }                        

//-----------------------Ends Here-----------------

var blockResponsechk=false;
var chkWin;
var chkWinMsg=&quot;&quot;;
var chkWinValue=false;
function blockClick()
            {
                 
                if(chkWin &amp;&amp; !chkWin.closed)
			{
			chkWinValue=true;
			if(chkWinMsg!=&quot;&quot;)
			{
			alert(chkWinMsg);
			}
			else {
			alert((&quot;Denomination window is not close. Please close it!&quot;).toUpperCase());
			}
			chkWin.focus();
			return false;
			}
			else {
				chkWinValue=false;
			}
               if((!document.getElementById(&quot;SaveLink&quot;)) || (!document.getElementById(&quot;SaveExitLink&quot;)))
                 {
                  if(blockResponsechk==true)
                  {
                        alert((&quot;Activity in Progress.Please wait...&quot;).toUpperCase());
                        return false;
                       
                  }
                  else
                  {                   	
                        blockResponsechk=true; 
                       setTimeout(&quot;setTimerReClick()&quot;,10000); // after 10 sec it&quot; , &quot;'&quot; , &quot;s call
                        return true;
                  }
                }
                
               
            }
           

	function setTimerReClick()
	{		
		blockResponsechk=false;
	}

//Disable right mouse click Script
//By Maximus (maximus@nsimail.com) w/ mods by DynamicDrive
//For full source code, visit http://www.dynamicdrive.com
//added by sunny to restrict refresh using ctrl +R
$(document).ready(function () {
    $(document).on(&quot;keydown&quot;, function(e) {
        e = e || window.event;
        if (e.ctrlKey) {
            var c = e.which || e.keyCode;
            if (c == 82) {
            	alert(message);
                e.preventDefault();
                e.stopPropagation();
                return false;
            }
        }
    });
});
//add end bby sunny
 var message=&quot;FUNCTION DISABLED!&quot;;

///////////////////////////////////
function clickIE4(){
if (event.button==2){
alert(message);
return false;
}
}

function clickNS4(e){
if (document.layers||document.getElementById&amp;&amp;!document.all){
if (e.which==2||e.which==3){
alert(message);
return false;
}
}
}

if (document.layers){
document.captureEvents(Event.MOUSEDOWN);
document.onmousedown=clickNS4;

}
else if (document.all&amp;&amp;!document.getElementById){
document.onmousedown=clickIE4;
}

document.oncontextmenu=new Function(&quot;alert(message);return false&quot;);




//This function changed by Ravikant for browser compatibility
function showKeyCode(e)
{
var src=src=e.srcElement;
if(src==null || src==&quot; , &quot;'&quot; , &quot;null&quot; , &quot;'&quot; , &quot; || src==undefined )
src=e.target;
var tag = src.tagName ? src.tagName.toUpperCase() : &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;; 
var typ = (tag == &quot; , &quot;'&quot; , &quot;INPUT&quot; , &quot;'&quot; , &quot;) ? src.type.toUpperCase() : &quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;; 
var isTextArea = (tag == &quot; , &quot;'&quot; , &quot;TEXTAREA&quot; , &quot;'&quot; , &quot;); 
var isTextField = ((tag == &quot; , &quot;'&quot; , &quot;INPUT&quot; , &quot;'&quot; , &quot;) &amp;&amp; (typ == &quot; , &quot;'&quot; , &quot;TEXT&quot; , &quot;'&quot; , &quot;)); 
var isPassField= ((tag == &quot; , &quot;'&quot; , &quot;INPUT&quot; , &quot;'&quot; , &quot;) &amp;&amp; (typ == &quot; , &quot;'&quot; , &quot;PASSWORD&quot; , &quot;'&quot; , &quot;)); 
var isText = isTextField || isTextArea || isPassField; 

var disabled = isText ? src.disabled : false; 
var readOnly = isText ? src.readOnly : false; 

var keycode =(window.event) ? event.keyCode : e.keyCode;

if(keycode == 116)
{
alert((&quot; , &quot;'&quot; , &quot;Function Disabled!&quot; , &quot;'&quot; , &quot;).toUpperCase());
event.keyCode = 0;
event.returnValue = false;
return false;
}
var browser=(navigator.userAgent).toLowerCase();
if(browser.indexOf(&quot;msie&quot;)>0){
if(!isText)
{
	if(keycode == 8)
	{
		alert((&quot; , &quot;'&quot; , &quot;Function Disabled!&quot; , &quot;'&quot; , &quot;).toUpperCase());
		event.keyCode = 0;
		event.returnValue = false;
		return false;
	}
}
}

if (disabled || readOnly) 
 { 

if(keycode == 8)
{

	alert((&quot; , &quot;'&quot; , &quot;Function Disabled!&quot; , &quot;'&quot; , &quot;).toUpperCase());
	event.keyCode = 0;
	event.returnValue = false;
	return false;
} 
}
}

/*Satrt Added For Bud Id:13340*/
var myWindow;
/*End Added For Bud Id:13340*/	
	function onBeforeUnloadAction()
 	{
		document.forms[0].action = &quot;userAuthAction.do?dispatchMethod=logout&quot;;
        document.forms[0].method = &quot;post&quot;;
        document.forms[0].submit();
   }
   
   window.onbeforeunload = function(e)
   {
   		if(window.event &amp;&amp; !e)
   		e=window.event;
   		
   		if((e.clientX&lt;0) ||(e.clientY&lt;0))
 		{
 			/*Satrt Added For Bud Id:13340*/
 			if (myWindow &amp;&amp; myWindow.closed) 
 			{
 		 		return onBeforeUnloadAction();
 			}
 			/*End Added For Bud Id:13340*/
 			
 			/*Satrt Comment For Bud Id:13340*/
 			
 			//return onBeforeUnloadAction();
 			
 			/*Satrt Comment For Bud Id:13340*/
 			
 		}
 	}; 
 	function debugAlert(alertMesg)
 	{
 	var debugAlert=&quot;N&quot;;
 	if(debugAlert==&quot;Y&quot;)
 	alert(alertMesg);
 	}
 	
 	function testing()
 	{
 	   // fore testing start
	
	 $.ajax({  
			type : &quot;post&quot;,   
			url : &quot;BroadcastServiceRequestServlet&quot;,   
			ontext: document.body,			
			success : function(response) {			
				  
			   }			
		}); 
	// for testing end
 	   
 	}

 
 


	
	
			
				
					  
						
				
					Hi NAVIND
					
					
				
				
				
					
				
				
					
				
				
				
				
				
					
					
						
							
							
							25-JUL-2023
							
						
					
						 
					
					
					
					
					NAVIND
					
				
					
				Logout
			
			
				
					
				
				
				
					
				
				
				
				
			
			
		
	
		
		 
		
  
	

	 

 
		$(window).on(&quot; , &quot;'&quot; , &quot;load&quot; , &quot;'&quot; , &quot;, function() {
			
			$(this).parents(&quot;tr&quot;).removeClass(&quot;cstmtr&quot;);
			$(document).on(&quot;click&quot;,&quot;#xyz table tr td a&quot;,function(){
			$(this).parents(&quot;tr&quot;).addClass(&quot;cstmtr&quot;);
			
			});
			$(document).on(&quot;blur&quot;,&quot;#xyz table tr td a&quot;,function(){
			$(this).parents(&quot;tr&quot;).removeClass(&quot;cstmtr&quot;);
			
			});
			$(&quot;#llmDashboardDiv table:nth-of-type(1) tr.tr_list_header td:nth-of-type(2)&quot;).css(&quot;width&quot;,&quot;40%&quot;);
			$(&quot;#llmDashboardDiv table:nth-of-type(2) td:nth-of-type(2)&quot;).css(&quot;width&quot;,&quot;40%&quot;);
			$(&quot;#llmDashboardDiv table:nth-of-type(2)  td:nth-child(3)&quot;).css(&quot;width&quot;,&quot;50%&quot;);
			//$(&quot;#llmDashboardDiv table:nth-of-type(1) tr.tr_list_header td:nth-of-type(4) b&quot;).css(&quot;padding-left&quot;,&quot;40%&quot;);
			//$(&quot;#llmDashboardDiv table:nth-of-type(1) tr&quot;).not(&quot; , &quot;'&quot; , &quot;:first&quot; , &quot;'&quot; , &quot;).not(&quot;.tr_list_header&quot;).find(&quot;td:nth-of-type(4)&quot;).wrapInner(&quot; , &quot;'&quot; , &quot;&lt;span style=&quot;position:relative;right:100px; text-align: right;display:block&quot;>&lt;/span>&quot; , &quot;'&quot; , &quot;);
			//$(&quot;#llmDashboardDiv table:nth-of-type(1) tr&quot;).not(&quot; , &quot;'&quot; , &quot;:first&quot; , &quot;'&quot; , &quot;).not(&quot;.tr_list_header&quot;).find(&quot;td:nth-of-type(3)&quot;).wrapInner(&quot; , &quot;'&quot; , &quot;&lt;span style=&quot;position:relative;right:59px; text-align: right;display:block&quot;>&lt;/span>&quot; , &quot;'&quot; , &quot;);
			//$(&quot;#llmDashboardDiv table:nth-of-type(2) tr&quot;).not(&quot; , &quot;'&quot; , &quot;:first&quot; , &quot;'&quot; , &quot;).not(&quot;.tr_list_header&quot;).find(&quot;td:nth-of-type(3)&quot;).wrapInner(&quot; , &quot;'&quot; , &quot;&lt;span style=&quot;position:relative;right:42%;text-align: right;display:block;width:72%&quot;>&lt;/span>&quot; , &quot;'&quot; , &quot;);
							var intRegex = /^\d+$/;
							var floatRegex = /^([\d\,\.]*)$/;
							var str1;
							var str = $(&quot; , &quot;'&quot; , &quot;body td&quot; , &quot;'&quot; , &quot;).each(function(){
							var str1 = $(this).text();
							  });
							if ($(&quot;#vetiBtn&quot;).length > 0) {
								$(&quot;.back_image_logo1&quot;).addClass(&quot;menu_add&quot;).find(&quot;img&quot;).attr(&quot;onclick&quot;, &quot;collapsePage()&quot;);
							} else {
								$(&quot;.back_image_logo1&quot;).removeClass(&quot;menu_add&quot;);
							}
							/* if ($.browser.version == 8.0) {
								var a = $.trim($(&quot;.imd_maker_space&quot;).text());
								if (a == &quot;&quot;) {
									/* console.log(&quot;ghkxgf&quot;); 
									$(&quot;.imd_maker_space,.imd_maker_user,.imd_maker_dashbord&quot;)
											.css(&quot;display&quot;, &quot;none&quot;);
									$(&quot;.imd_maker_tab&quot;).css(&quot;width&quot;, &quot;0.1%&quot;);
									$(&quot;.imd_maker_dashbord&quot;).css(&quot;width&quot;,
											&quot;1.4%&quot;);
									$(&quot;.imd_maker_user&quot;).css(&quot;width&quot;, &quot;0.8%&quot;);
									$(
											&quot;.menu-dashboard,.td_under_strip,.menu-save &quot;)
											.css(&quot;width&quot;, &quot;10%&quot;);

								}

							} */

						});
	 
	
		$(&quot;.userr&quot;).click(function() {
			$(this).parents(&quot;.username_icon&quot;).siblings(
			&quot;.username_tooltip&quot;).toggle(&quot;slow&quot;);
		});
		divHeight=&quot;&quot;;
		divHeight1=&quot;&quot;;
		$(window).on(&quot; , &quot;'&quot; , &quot;load&quot; , &quot;'&quot; , &quot;,function(){
			$(&quot;.text_footer&quot;).removeClass(&quot;relativefooter&quot;);
			if($(&quot;body&quot;).css(&quot;overflow-y&quot;)==&quot;scroll&quot;){$(&quot;.text_footer&quot;).addClass(&quot;relativefooter&quot;);}
			else{$(&quot;.text_footer&quot;).removeClass(&quot;relativefooter&quot;);}
    		$(&quot; , &quot;'&quot; , &quot;body&quot; , &quot;'&quot; , &quot;).css(&quot; , &quot;'&quot; , &quot;margin-top&quot; , &quot;'&quot; , &quot;, divHeight+divHeight1+&quot; , &quot;'&quot; , &quot;px&quot; , &quot;'&quot; , &quot;);
			$(&quot;tr td input&quot;).each(function(){
				if($(this).attr(&quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;) == &quot; , &quot;'&quot; , &quot;readonly&quot; , &quot;'&quot; , &quot;){
				$(this).addClass(&quot;readonly_text&quot;);  }
			});
			$(&quot;input[name=&quot; , &quot;'&quot; , &quot;addAddressButton&quot; , &quot;'&quot; , &quot;]&quot;).click(function(){
				$(this).parents(&quot;.add_new_address_dtl&quot;).next(&quot;table&quot;).find(&quot;.readonly_text&quot;).removeClass(&quot;readonly_text&quot;);
			});
		});
	
		$(&quot;[name=&quot; , &quot;'&quot; , &quot;prospectListHeading&quot; , &quot;'&quot; , &quot;]&quot;).attr(&quot;id&quot;,&quot;prosListTable&quot;);
		$(&quot;#prosListTable tr td&quot;).eq(2).addClass(&quot;prosListTable_adjust2&quot;);
		$(&quot;#prosListTable tr td&quot;).eq(3).addClass(&quot;prosListTable_adjust3&quot;);
		$(window).on(&quot; , &quot;'&quot; , &quot;load&quot; , &quot;'&quot; , &quot;,function() {
		
			if($(&quot;.tt&quot;).text().trim()==&quot;&quot;){$(&quot;.tt&quot;).fadeOut();}
			if($(&quot;#uu&quot;).text().trim()==&quot;&quot;){$(&quot;#uu&quot;).fadeOut();}
			if($(&quot;#SDNDataId&quot;).text().trim()==&quot;&quot;){$(&quot;#SDNDataId&quot;).fadeOut();}
			$(&quot;#prosListTable tr td&quot;).eq(0).addClass(&quot;prosListTable_adjust1&quot;);
			//$(&quot;#llmDashboardDiv .static_info:nth-of-type(2) tr:nth-of-type(2) td:nth-of-type(3) b&quot;).css(&quot;padding-left&quot;,&quot;23%&quot;);
			//$(&quot;#llmDashboardDiv .static_info:nth-of-type(1) tr:nth-of-type(2) td:nth-of-type(3) b&quot;).css(&quot;padding-left&quot;,&quot;56%&quot;);
			$(&quot;#llmDashboardDiv .static_info:nth-of-type(1) tr td:nth-of-type(1)&quot;).attr(&quot;style&quot;,&quot;width:1%&quot;);
			$(&quot;#llmDashboardDiv table:nth-of-type(1) tr td:nth-of-type(1)&quot;).css(&quot;width&quot;,&quot;3%&quot;);
			$(&quot;#llmDashboardDiv .static_info:nth-of-type(1) tr td:nth-of-type(1) img&quot;).css({&quot;float&quot;:&quot;right&quot;,&quot;margin-right&quot;:&quot;4px&quot;});
			$(&quot;#llmDashboardDiv table:nth-of-type(2) tr td:nth-of-type(1)&quot;).css(&quot;width&quot;,&quot;3%&quot;);
			$(&quot;#llmDashboardDiv .static_info:nth-of-type(2) tr td:nth-of-type(1) img&quot;).css({&quot;float&quot;:&quot;right&quot;,&quot;margin-right&quot;:&quot;4px&quot;});
			$(&quot;.div_applisttable1  tr:not(:first-child)&quot;).each(function(){ 
				var linkhref = $(this).find(&quot;td:nth-child(5) a&quot;).attr(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;);
				$(this).find(&quot;td:nth-child(3)&quot;).wrapInner(&quot; , &quot;'&quot; , &quot;&lt;a href=&quot; , &quot;'&quot; , &quot;+linkhref +&quot; , &quot;'&quot; , &quot;>&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;&lt;/a>&quot; , &quot;'&quot; , &quot;);
			});
			var appname_txt = $(&quot;.applist_imd td:nth-child(3)&quot;).text();
			$(&quot;.applist_imd td:nth-child(3)&quot;).html(appname_txt);
			var divTest = $(&quot;#fset_one&quot;).height();
			var flagg1 = false;
 			$(&quot;.menu_tab tbody td a&quot;).width($(&quot;.menu_tab tbody td a&quot;).parent(&quot;td&quot;).outerWidth()-5);
			$(&quot;#verticalMenu&quot;).css({
							&quot;width&quot; : &quot;94% !important&quot;,
							&quot;position&quot; : &quot;relative&quot;,
							&quot;left&quot; : &quot;143px !important&quot;,
							&quot;overflow-y&quot; : &quot;auto&quot;,
						});

});


	 	
	
	


			








	
			
			
				
					MANUAL ADVICE MAKER
				
			
			
			
    
	
			
		
		 
			My Dashboard
		 
	
		
	        		
		
			
					
				 

			
            
			    
                
				
				
				
				
				
				
				  
				
				
				
				
				
				
				
				  
				   
				   
					
					My Worklist
					
					
					   
				
				 
				 
				 
				
				
				
				
				
				
				
				
				
				
				
				
				
				
			
				
				
				

				
			  
	
				
		    
		    





						
						
						
						
						
						
						
						
						
						
						
						
						
						
						  
						
						
						
						 
						
						
						
						
						
						 Save
						 
						
						
						
						
						
						
						
						
						
						
						
						
						
						
						
						
						
						
						
						
						
						

						
										
								    
								    
						


						
						
						
						
							
						 
						
						 
						 
						
						
						 Save &amp; Exit
							 
							
							
							
							
						
						
						
						

						
						
						
						
						
						


				
			
			
		
		
								
		
	


	      











var inactiveInterval = 1800
var loanAmt = &quot;&quot;;

	//This function is changed by Ravikant for browser compatibility.
	function callWfp()
	{
		var sessionStatus = checkinterval(inactiveInterval);		
		if(sessionStatus==&quot;Y&quot;)
		{	 
			var browser=(navigator.userAgent).toLowerCase();
			//alert(browser);
			if(browser.indexOf(&quot;chrome&quot;)>0)
			{	
				var targetWin = window.open(&quot;workFlowProc.do?actionPerformed=displayWorkFlowPage&amp;rurl=&quot;+Math.random(),&quot;workFlow&quot;,&quot;dialogWidth:1000px;dialogHeight:500px&quot;);
				targetWin.focus();
			}else{
				window.showModalDialog(&quot;workFlowProc.do?actionPerformed=displayWorkFlowPage&amp;rurl=&quot;+Math.random(),&quot;workFlow&quot;,&quot;dialogWidth:1000px;dialogHeight:500px&quot;);
			}
		}
		else
		{
			
			window.location=&quot;userAuthAction.do?dispatchMethod=logout&quot;;
		}
	}
	
	function showNotePad()
	{
		var sessionStatus = checkinterval(inactiveInterval);		
		if(sessionStatus==&quot;Y&quot;)
		{	 
			var browser=(navigator.userAgent).toLowerCase();
			//alert(browser);
			if(browser.indexOf(&quot;chrome&quot;)>0)
			{	
				var targetWin = window.open(&quot;notepadAction.do?actionPerformed=displayNotePadInfo&amp;staticCall=Y&amp;rurl=&quot;+Math.random(),&quot;workFlow&quot;,&quot;dialogWidth:1030px;dialogHeight:600px&quot;);
				targetWin.focus();
			}else{
				window.showModalDialog(&quot;notepadAction.do?actionPerformed=displayNotePadInfo&amp;staticCall=Y&amp;rurl=&quot;+Math.random(),&quot;workFlow&quot;,&quot;dialogWidth:1030px;dialogHeight:600px&quot;);
			}
		}
		else
		{
			
			window.location=&quot;userAuthAction.do?dispatchMethod=logout&quot;;
		}
	}
	
    	/* function for the  Portable Search Div Hide And Show Button(+/-)  date 14 /06/2012  By Ambar Gupta */
	function showdiv()
	{	
	 
           var Lgd = document.getElementById(&quot;searchLgd&quot;);
        //mayank Agrawal for compatiblity
            if (document.getElementById(&quot;one&quot;).style.display == &quot;block&quot; || document.getElementById(&quot;one&quot;).style.display==&quot;&quot;)
            {   
                
                 
                  
                  			   
				    document.getElementById(&quot;imageUpload&quot;).style.display = &quot;block&quot;;
				  
				  
				 
				
				document.getElementById(&quot;one&quot;).style.display = &quot;none&quot;; 
				var temp=document.getElementsByName(&quot;btn_one&quot;);
				
				
				document.getElementById(&quot;fset_one&quot;).style.border= &quot;0px solid #83b0ec&quot;;
				Lgd.innerHTML = &quot; , &quot;'&quot; , &quot;&lt;input type=&quot;button&quot; class=&quot;imageBottonDown&quot;  name=&quot;btn_one&quot; id=&quot;btn_one&quot; onclick=&quot;javascript:showdiv()&quot;  title=&quot;Maximize the Common Information&quot; style=&quot;cursor: pointer;border: none&quot;>&lt;/input>&amp;nbsp;Static Info&quot; , &quot;'&quot; , &quot;;
			}
            else if (document.getElementById(&quot;one&quot;).style.display == &quot;none&quot;)
			{ 
			    
				document.getElementById(&quot;one&quot;).style.display = &quot;block&quot;;
			    
                 
                   
                  			   
				    document.getElementById(&quot;imageUpload&quot;).style.display = &quot;block&quot;;
				  
				  
				 
				
				var temp=document.getElementsByName(&quot;btn_one&quot;);
				
				document.getElementById(&quot;fset_one&quot;).style.border= &quot;1px solid #83b0ec&quot;;
				Lgd.innerHTML = &quot; , &quot;'&quot; , &quot;&lt;input type=&quot;button&quot; class=&quot;imageBottonUp&quot;  name=&quot;btn_one&quot; id=&quot;btn_one&quot;  onclick=&quot;javascript:showdiv()&quot;  title=&quot;Minimize the Common Information&quot; style=&quot;cursor: pointer;border: none&quot;>&lt;/input>&amp;nbsp;Static Info&quot; , &quot;'&quot; , &quot;;
			}	
						
	}
	      /*   Function End Here */	
// 1.0.0.5 start 	      
function rentalPayScheduleReport()
  	{
	window.open(&quot;rentalPayScheduleReport.sprg&quot;);	
  	}	      	
// 1.0.0.5 end 	

//1.0.0.12 start
 function getReschHistScreen()
{
	 
	var paramList=&quot; , &quot;'&quot; , &quot;1000008259&quot; , &quot;'&quot; , &quot;;
    var url =   &quot;dynamicWindow.do?actionPerformed=dynamicWindow&amp;windowId=1000000005&amp;paramList=&quot;+paramList+&quot;&quot;;
	   
	   var self=window.open(url,&quot;dynamicWindow&quot;,&quot;width=800px,height=500px,left=400px,titlebar=yes,scrollbars=yes,toolbar=no,maximize=no,menubar=no,minimize=no,statusbar=no&quot;);
	   
} 
//1.0.0.12 end








 

 


  	   
	

		
				PROSPECT NO	
			DMFIN1000008259
		
			
			
			CUSTCODE		
			CNCIMF000003205
		
		
		
			CUST NAME
			ROHAN  TESTQA
		
		
		
		
		
		
		
		
		CUST LIMIT CODE
			LMCST0000006150
			
			
			
			
		QUOTATION CODE
			QU00022832
			
			
			
			
			
		
			
			GROUP NAME	
			
			PROJECT TEST RO
	    
			
		
			CAPITALISED FLAG
			Y
			
			
			
			
			PRODUCT
			Finance Lease
			
		
			
			SCHEME
			ONLY FIXED FINA...ONLY FIXED FINANCE LEASE
		
		
			
			ASSET CATEGORY
			VEHICLE
		
			
			AGREEMENT TYPE
			BI PARTY
		
		
		
		
		
			SANCTIONED AMT	
			115.00
		
		
		
			
             LEASE SANCTIONE...LEASE SANCTIONED DATE
			13-JAN-2023			
			
		
		
		
		
		
		
			
			
			LEASE RENTAL AMT	
				
			12.00
		
		
			
			RENTAL TYPE
			EQUATED
		
		
		
  
	  
			
			ENTITY TYPE
			INDIVIDUAL
		
			
		
		
		
			
				NO. OF INSTALMENTS
				10
			
		
		
			BRANCH
			HEAD OFFICE
		
	
		
			APPLICATION FOR...APPLICATION FORM NO.
			APPFORM0008259
		
		
		
		
			
			CASHFLOW IRR
			
			5
		
		
		
		
			
			RENTAL START DATE
			
			07-MAR-2023
		
		
		
		
	
	
	
		
		
		
		
			DM STATUS
		
		
		
		
		DISBURSED
		
		
		
		
		
			LEASE APPLICATI...LEASE APPLICATION DATE
		
		
			13-JAN-2023
		
		
		
		
		
		
			
			RENTAL FREQUENCY
			
			MONTHLY
		
		
		
			FORECLOSURE METHOD
			BOOK VALUE
			
			
		
			NPA STAGE
			REGULAR
		
		
		 
	   
    
	
		DAYS SINCE CUST...DAYS SINCE CUSTOMER SIGNED
		
	
	
		
		DAYS IN CURRENT...DAYS IN CURRENT STAGE
		
	
	
	
      
  	
	     
    
				
		
		
	
	    
   
	  
		
	
		
		
		
			TRANSFERED FROM DM
			
			
			
			
			
		
			RV TYPE
			100+1
			
			
			
		
			RV AMOUNT
			                   5.00
			
			
			
			
				
			
			
					    
		    
			
			RESCHEDULED
			
			N
			
			
			
			
			
			
		
			NOTEPAD
			
		
			
		
	
	
	
		
		
		
		   
	
  
  
  
  
  
  
  
 
  
		
			
		
		  
		
 
		   
		   		


 
  
     
     
    
	 
	
	
	
	
	
    



		


	 		 
	  	 




 



	$(document).ready(function(){

		var linkhref = $(&quot;.noti_link span a&quot;).attr(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;);
				$(&quot;.prose2 span&quot;).wrapInner(&quot; , &quot;'&quot; , &quot;&lt;a href=&quot; , &quot;'&quot; , &quot;+linkhref +&quot; , &quot;'&quot; , &quot;>&quot; , &quot;'&quot; , &quot; + &quot; , &quot;'&quot; , &quot;&lt;/a>&quot; , &quot;'&quot; , &quot;);
				
	
				var heightDiv = &quot;80px&quot;;
				
			
				$(&quot;#one&quot;).addClass(&quot;onecustom&quot;);
				$(&quot;#one&quot;).toggleClass(&quot;tgling&quot;).css(&quot;height&quot;,&quot;30px&quot;);
				$(document).on(&quot;click&quot;,&quot;.tgl_nbtn&quot;,function(){
					$(&quot;#one&quot;).toggleClass(&quot;tgling&quot;);
		//1.0.0.4 start			
				if(document.getElementById(&quot;customerLimitStaticInfo&quot;))
				 {
				   heightDiv= Number($(&quot;#customerLimitStaticInfo&quot;).height())+20;
				 }
				 else  if(document.getElementById(&quot;groupStaticInfo&quot;))
				 {
				   heightDiv= Number($(&quot;#groupStaticInfo&quot;).height())+20;
				 }
				  else  if(document.getElementById(&quot;customerStaticInfo&quot;))
				 {
				   heightDiv= Number($(&quot;#customerStaticInfo&quot;).height())+20;
				 }
				 else  if(document.getElementById(&quot;cvLimitStaticInfo&quot;))
				 {
				   heightDiv= Number($(&quot;#cvLimitStaticInfo&quot;).height())+20;
				 }
				 else  if(document.getElementById(&quot;losStaticInfo&quot;))
				 {
				   heightDiv= Number($(&quot;#losStaticInfo&quot;).height())+40;
				 }
			//1.0.0.4 end			
					$(&quot;#one&quot;).css(&quot;height&quot;,heightDiv);
					$(&quot;#one.tgling&quot;).css(&quot;height&quot;,&quot;30px&quot;);
					$(this).toggleClass(&quot;tglingSpan&quot;);
				});

			$(&quot;#fset_one .static_info_head&quot;).each(function(){
			if($(this).text().trim().length>=20) 
			{ 
			var originText1=$(this).text();   
			var cropText1=originText1.substring(0,15); 
			$(this).text(cropText1+&quot;...&quot;); 
			$(this).after(&quot;&lt;span class=&quot; , &quot;'&quot; , &quot;hoverStaticInfo&quot; , &quot;'&quot; , &quot; style=&quot; , &quot;'&quot; , &quot;display:none&quot; , &quot;'&quot; , &quot;>&quot;+originText1+&quot;&lt;/span>&quot;);$(this).addClass(&quot;positioned_div&quot;);
			}
		

});



	
	});

	
		
			


	
		
		
	var contextPath = &quot;/lease&quot;;

		
		
		
		
		
/* All Changes have done by Ambar Gupta for Persist the Stae of menu link and also have Show hide functionalty */
var isCollapsed = false;
//Changed by Ravikant for browser compatibility on 29-Dec-14.
function collapsePage()
{
	var menuDiv = document.getElementById(&quot;verticalMenu&quot;);
	var barDiv = document.getElementById(&quot;linkTD&quot;);
	var vetiTd = document.getElementById(&quot;vetiTd&quot;);
	if(isCollapsed)
	{
		menuDiv.style.display = &quot;block&quot;;
// 		document.getElementById(&quot; , &quot;'&quot; , &quot;dasiTd&quot; , &quot;'&quot; , &quot;).style.width = &quot;80%&quot;;
// 		document.getElementById(&quot; , &quot;'&quot; , &quot;vetiTd&quot; , &quot;'&quot; , &quot;).style.width = &quot;20%&quot;;
		setDimension(&quot;1&quot;);
		
		barDiv.innerHTML = &quot; , &quot;'&quot; , &quot;&amp;nbsp;&amp;nbsp;&amp;nbsp;&amp;nbsp;VERTICAL MENU &amp;nbsp;&amp;nbsp;&amp;nbsp;&amp;nbsp;&amp;nbsp;&amp;nbsp;&amp;nbsp;&amp;nbsp;&lt;input type=&quot;button&quot; class=&quot;imageBottonLeft&quot;  onclick=&quot;collapsePage()&quot; title=&quot;Minimize Vertical Menu&quot; style=&quot;cursor: pointer;border:none;margin-bottom:0px;margin-top:0px;align:right;&quot; id=&quot;vetiBtn&quot;>&quot; , &quot;'&quot; , &quot;;
		isCollapsed = false;
		vetiTd.style.backgroundImage = &quot;&quot;;
	}
	else
	{	
		barDiv.innerHTML = &quot; , &quot;'&quot; , &quot;&lt;input type=&quot;button&quot; class=&quot;imageBottonRight&quot;  onclick=&quot;collapsePage()&quot; title=&quot;Maximize Vertical Menu&quot; style=&quot;cursor: pointer;border: none;margin-bottom:0px;margin-top:0px;&quot; id=&quot;vetiBtn&quot; align=&quot;right&quot;>&quot; , &quot;'&quot; , &quot;;
		menuDiv.style.display = &quot;none&quot;;
// 		document.getElementById(&quot; , &quot;'&quot; , &quot;dasiTd&quot; , &quot;'&quot; , &quot;).style.width = &quot;99%&quot;;
// 		document.getElementById(&quot; , &quot;'&quot; , &quot;vetiTd&quot; , &quot;'&quot; , &quot;).style.width = &quot;1%&quot;;
		setDimension(&quot;2&quot;);
		newImage = &quot;url(images/VERTICALMENUBAR.gif)&quot;;
		vetiTd.style.backgroundImage = newImage;
		isCollapsed = true;
	}
	menuDiv.style.width=&quot;230&quot;;
	//menuDiv.style.height=&quot;100%&quot;;
	menuDiv.style.overflow=&quot;auto&quot;;
}
function changecolor1()
	{
	   var barBtn = document.getElementById(&quot;vetiBtn&quot;);
	   barBtn.style.backgroundColor=&quot; , &quot;'&quot; , &quot;#00FF00&quot; , &quot;'&quot; , &quot;;
	}
	function changecolor()
	{ 
	   
	   var barBtn = document.getElementById(&quot;vetiBtn&quot;);
	   barBtn.style.backgroundColor=&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;;
	}
	//Added by Ravikant for browser compatibility on 29-Dec-14.
	function setDimension(d)
	{
		var x = screen.width;
		var y = screen.height;
		if(d == &quot;1&quot;)
		{
			//document.getElementById(&quot;vetiTd&quot;).style.width = (x*0.2) + &quot;px&quot;; 
		   // document.getElementById(&quot;dasiTd&quot;).style.width = (x-(x*0.2)) + &quot;px&quot;;	
		    
		   // document.getElementById(&quot;vetiTd&quot;).style.height = (y-(y*0.01)) + &quot;px&quot;;  
		   // document.getElementById(&quot;dasiTd&quot;).style.height = (y-(y*0.01)) + &quot;px&quot;;   
	    }
	    else if(d == &quot;2&quot;)
	    {
	    	//document.getElementById(&quot;vetiTd&quot;).style.width = (x*0.01) + &quot;px&quot;; 
		   // document.getElementById(&quot;dasiTd&quot;).style.width = (x-(x*0.01)) + &quot;px&quot;;
		    
		   // document.getElementById(&quot;vetiTd&quot;).style.height = (y-(y*0.01)) + &quot;px&quot;;  
		  //  document.getElementById(&quot;dasiTd&quot;).style.height = (y-(y*0.01)) + &quot;px&quot;;  
	    }
	    //alert(document.getElementById(&quot;vetiTd&quot;).style.width+&quot; &quot;+document.getElementById(&quot;dasiTd&quot;).style.width+&quot; &quot;+document.getElementById(&quot;dasiTd&quot;).style.height);
	}

	



		
			

			
				
				
			
			
				
				UTILITYCHANGE PASSWORDCUSTOMERNEWCUSTOMERPENDING ACTIVITIESAPPLICANTDEDUPENOTEPADDOCUMENTCR REVIEWCUSTOMER EDITCUSTOMER VIEWERGROUPNEW GROUPGROUP EDITLIMIT(LEASE)CUSTOMER LIMITNEWCUSTOMER LIMITPENDING ACTIVITIESEDITCR DECISIONCUSTOMER LIMIT VIEWERGROUP LIMITGROUP LIMIT VIEWERQUOTATIONNEWLEASE PROPOSALPENDING ACTIVITIESASSET DETAILSQUOTATIONINSURANCECASHFLOWREGISTRATION DETAILSACCEPTANCEACQUISITIONVIEWERDM QUOTATIONNEWDM APPLICATIONPENDING ACTIVITIESASSET DETAILSQUOTATIONCASHFLOWDM OFFLINE ACTIONDM SANCTIONCAPITALISATION  MAKERDM CANCELLATIONMAKERVIEWERPAYMENTMAKERVIEWERDM VIEWERLMSINVOICEINVOICEDM PDEMAKERRECEIPTMAKERPAYMENT HISTORYRECEIPT CANCELLATIONMAKERVIEWERREFUNDMAKERVIEWERMANUAL ADVICEMAKERKNOCK OFFMAKERVIEWERINSTR MANAGEMENTPDC/ECS GENERATEMAKERPDC/ECS EDITMAKERVIEWERWAIVE OFFMAKERVIEWERCLOSE / FORECLOSEMAKERVIEWERBATCH PROCESSGENERATIONRESCHEDULING CASEMAKERREPORTSCREDIT OPERATION REPORTANNEXES OLAUDIT CONFIRMATIONBOARD OF RESOLUTIONBOARD RESOLUTION CORPORATE GUARANTORCANCELLATION SALARY DEDUCTION FORMCANCELLATION STANDING ORDER FORMCERTIFICAT DE GAGE SECOND HANDCHANGE IN ENGINE NUMBERCHANGE IN REGISTRATION NUMBERCHANGE OF NAME OF HORSEPOWERCORPORATE GUARANTEECOVER LETTERDIRECT DEBIT FORMHORSEPOWER FULLLEASE AGREEMENTLEASE AGREEMENT OLLETTER OF UNDERTAKINGLOST HORSEPOWERNO LIABILITY LETTEROFFER LETTERREPAYMENT SCHEDULERIGHT OF SET OFF LETTERRV LETTERRV SALES DEED AND CERTIFICATE OF GAGESALARY DEDUCTION FORMSALES DEEDSETTLEMENT LETTERSTANDING ORDER FORMSUBORDINATION OF SHAREHOLDERS LOANSUPPLIER LETTERACCOUNTING REPORTACCRUAL REPORTASSET RECEIPT REPORTAUTO RESCHEDULE STATUS REPORTBASE_RATE_CHANGE_TRACK_REPORTCLIENT MIS REPORTCUSTOMER DETAIL REPORTDOCUMENT PENDING AND EXPIRY REPORTINSURANCE REPORTINVOICE REPORTLEAD FOLLOW UPLEAD STATUSLEASE REGISTRATION TRACKINGLIMIT EXPIRYLPI REPORTMATURING AGREEMENTS REPORTOUTSTANDING DUE REPORTRV DUE REPORTSCHEDULED REPORTSTRANSACTION REPORTWAIVER_REPORTMANUAL ADVICELEASE PROPOSALPO REPORTQUOTATION DETAIL REPORTRESCHEDULING REPORTBATCH DTL REPORTSTATEMENT OF ACCOUNTBATCH PRESENTATION REPORTTRANSACTION REPORTTRIAL BALANCEVENDOR PAYMENT REPORTBATCH STATUS REPORTBATCH UPLOAD ISSUES REPORTBULK INVOICE REPORTCOVERNOTE REPORTAGEING REPORTBRANCH WISE SUMMARYUSER WISE SUMMARYCUSTOMER LIMIT REPORTFORECLOSURE REPORTALCO REPORTGROUP CUSTOMER REPORTGROUP LIMIT REPORTLEASE CAM REPORTLEASE CREDIT NOTEVAT INVOICEMASTERSMAKERSALESMANAGERAUTHORBATCH UPLOADMAKERNEGATIVE LISTSDNTALIBANAL-QAIDAMCIBCAUTION LISTAP INT GL MAP CONFIGBULK RECEIPTINSTRUMENT MANAGEMENTLEASE REGISTRATION TRACKINGOFFLINE INSURANCE DETAILSPDC/ECSQT FLEET DETAIL UPLOADLEASE ASSET PRICE MST UPLOADLEASE ASSET VENDOR UPLOADLEASE VENDOR GSTIN UPLOADMANUAL VOUCHER UPLOADCUSTOMER INVOICE CONFIG UPLOADLEASE BATTERY PRICE UPLOADLEASE TYRE PRICE UPLOADMANUAL ADVICE UPLOADLEASE ASSET CESSMAPPING UPLOADLEASE ASSET VARIANT UPLOADLEASE FC AMNT FORMULA UPLOADLEASE SERVICE CHRG AMT UPLOADLEASE VENDOR BANK UPLOADLEASE VENDOR MAPPING UPLOADLEASE MAINT SLABS UPLOADBULK CLOSURECHARGE X GST UPLOADLEASE RESIDUAL VALUE UPLOADAUTHORNEGATIVE LISTCAUTION LISTAP INT GL MAP CONFIGBULK CLOSUREBULK RECEIPTCUSTOMER RECEIPTLEASE ASSET PRICE MST UPLOADPDC/ECSLEASE VENDOR BANK UPLOADMANUAL ADVICE UPLOADLEASE ASSET VARIANT UPLOADLEASE FC AMNT FORMULA UPLOADLEASE SERVICE CHRG AMT UPLOADLEASE ASSET VENDOR UPLOADLEASE VENDOR GSTIN UPLOADMANUAL VOUCHER UPLOAD AUTHORLEASE BATTERY PRICE UPLOADLEASE TYRE PRICE UPLOADLEASE MAINT SLABS UPLOADCHARGE X GST UPLOADLEASE RESIDUAL VALUE UPLOADVIEWERNEGATIVE LISTSDNTALIBANAL-QAIDAMCIBCAUTION LISTAP INT GL MAP CONFIGLEASE REGISTRATION TRACKINGOFFLINE INSURANCE DETAILSLEASE ASSET PRICE MST UPLOADMANUAL VOUCHER UPLOADCUSTOMER INVOICE CONFIG UPLOADMANUAL ADVICE UPLOADLEASE ASSET VARIANT UPLOADLEASE FC AMNT FORMULA UPLOADLEASE SERVICE CHRG AMT UPLOADLEASE VENDOR BANK UPLOADLEASE VENDOR MAPPING UPLOADBULK RECEIPTCUSTOMER RECEIPT VIEWERINSTRUMENT MANAGEMENT VIEWERRESCH VIEWERBULK CLOSURE VIEWERPDC/ECSLEASE ASSET VENDOR UPLOADLEASE VENDOR GSTIN UPLOADLEASE BATTERY PRICE UPLOADLEASE TYRE PRICE UPLOADLEASE MAINT SLABS UPLOADCHARGE X GST UPLOADLEASE RESIDUAL VALUE UPLOAD
					
					var menuDataXml = &quot;&lt;ROWSET> &lt;ROW>  &lt;ACTION_ID>1000000001&lt;/ACTION_ID>  &lt;PARENT_ID/>  &lt;DISPLAY_NAME>UTILITY&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>UTILITY&lt;/MENU_TYPE>  &lt;SEQUENCE>10&lt;/SEQUENCE>  &lt;SCREEN_NAME>UTILITY&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>10&lt;/SEQ_NO>  &lt;PATH>-110&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000002&lt;/ACTION_ID>  &lt;PARENT_ID>1000000001&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CHANGE PASSWORD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>forwardFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000002&amp;amp;screenName=CHANGE PASSWORD&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>CHANGE PASSWORD&lt;/MENU_TYPE>  &lt;SEQUENCE>2&lt;/SEQUENCE>  &lt;SCREEN_NAME>AAA&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>changePassword.do?actionPerformed=displayChangePassword&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>2&lt;/SEQ_NO>  &lt;PATH>-110-22&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000002041&lt;/ACTION_ID>  &lt;PARENT_ID/>  &lt;DISPLAY_NAME>CUSTOMER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>CUSTOMER&lt;/MENU_TYPE>  &lt;SEQUENCE>11&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>11&lt;/SEQ_NO>  &lt;PATH>-111&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000010289&lt;/ACTION_ID>  &lt;PARENT_ID>1000002041&lt;/PARENT_ID>  &lt;DISPLAY_NAME>NEW  &lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>NEW&lt;/MENU_TYPE>  &lt;SEQUENCE>22&lt;/SEQUENCE>  &lt;SCREEN_NAME>NEW&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>22&lt;/SEQ_NO>  &lt;PATH>-111-222&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000002042&lt;/ACTION_ID>  &lt;PARENT_ID>1000010289&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>NewCustomerDetails.do?actionPerformed=displayCustomer&amp;amp;screenId=1000002042&amp;amp;screenName=CUSTOMER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>CUSTOMER&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>NewCustomerDetails.do?actionPerformed=displayCustomer&amp;amp;screenId=1000002042&amp;amp;screenName=CUSTOMER&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>CustomerDetails&lt;/REQUEST_PAGE>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-111-222-33&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000002049&lt;/ACTION_ID>  &lt;PARENT_ID>1000002041&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER EDIT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>applicantListFrmNavAction.do?actionPerformed=displayApplicantListInfo&amp;amp;screenId=1000002049&amp;amp;screenName=CUSTOMER EDIT&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>CUSTOMER&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER EDIT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>applicantList.do?actionPerformed=displayCustomerList&amp;amp;screenId=1000000079&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-111-23&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000002043&lt;/ACTION_ID>  &lt;PARENT_ID>1000002041&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>applicantListFrmNavAction.do?actionPerformed=displayApplicantListInfo&amp;amp;screenId=1000002043&amp;amp;screenName=CUSTOMER VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>CUSTOMER&lt;/MENU_TYPE>  &lt;SEQUENCE>4&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>applicantList.do?actionPerformed=displayCustomerList&amp;amp;screenId=1000002043&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>4&lt;/SEQ_NO>  &lt;PATH>-111-24&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000010288&lt;/ACTION_ID>  &lt;PARENT_ID>1000002041&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PENDING ACTIVITIES &lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PENDING ACTIVITIES&lt;/MENU_TYPE>  &lt;SEQUENCE>5&lt;/SEQUENCE>  &lt;SCREEN_NAME>PENDING ACTIVITIES&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>5&lt;/SEQ_NO>  &lt;PATH>-111-25&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000010283&lt;/ACTION_ID>  &lt;PARENT_ID>1000010288&lt;/PARENT_ID>  &lt;DISPLAY_NAME>APPLICANT &lt;/DISPLAY_NAME>  &lt;ACTION_LINK>applicantListFrmNavAction.do?actionPerformed=displayApplicantListInfo&amp;amp;screenId=1000010283&amp;amp;screenName=CUSTOMER DETAIL&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>APPLICANT DETAIL&lt;/MENU_TYPE>  &lt;SEQUENCE>11&lt;/SEQUENCE>  &lt;SCREEN_NAME>APPLICANT DETAIL&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>applicantList.do?actionPerformed=displayCustomerList&amp;amp;screenId=1000000079&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>11&lt;/SEQ_NO>  &lt;PATH>-111-25-311&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000010284&lt;/ACTION_ID>  &lt;PARENT_ID>1000010288&lt;/PARENT_ID>  &lt;DISPLAY_NAME>DEDUPE &lt;/DISPLAY_NAME>  &lt;ACTION_LINK>applicantListFrmNavAction.do?actionPerformed=displayApplicantListInfo&amp;amp;screenId=1000010284&amp;amp;screenName=CUSTOMER DETAIL&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>DEDUPE&lt;/MENU_TYPE>  &lt;SEQUENCE>12&lt;/SEQUENCE>  &lt;SCREEN_NAME>DEDUPE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>dedupe.do?actionPerformed=displayDedupeCustomerList&amp;amp;screenId=1000000080&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>12&lt;/SEQ_NO>  &lt;PATH>-111-25-312&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000010285&lt;/ACTION_ID>  &lt;PARENT_ID>1000010288&lt;/PARENT_ID>  &lt;DISPLAY_NAME>NOTEPAD &lt;/DISPLAY_NAME>  &lt;ACTION_LINK>applicantListFrmNavAction.do?actionPerformed=displayApplicantListInfo&amp;amp;screenId=1000010285&amp;amp;screenName=CUSTOMER DETAIL&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>NOTEPAD&lt;/MENU_TYPE>  &lt;SEQUENCE>13&lt;/SEQUENCE>  &lt;SCREEN_NAME>NOTEPAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>notepadAction.do?actionPerformed=displayNotePadInfo&amp;amp;screenId=1000000084&amp;amp;mode=E&amp;amp;disName=NOTEPAD&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>13&lt;/SEQ_NO>  &lt;PATH>-111-25-313&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000010287&lt;/ACTION_ID>  &lt;PARENT_ID>1000010288&lt;/PARENT_ID>  &lt;DISPLAY_NAME>DOCUMENT &lt;/DISPLAY_NAME>  &lt;ACTION_LINK>applicantListFrmNavAction.do?actionPerformed=displayApplicantListInfo&amp;amp;screenId=1000010287&amp;amp;screenName=CUSTOMER DETAIL&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>DOCUMENT&lt;/MENU_TYPE>  &lt;SEQUENCE>15&lt;/SEQUENCE>  &lt;SCREEN_NAME>DOCUMENTS&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>documentEntryAction.do?actionPerformed=displayDocumentEntry&amp;amp;screenId=1000000082&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>15&lt;/SEQ_NO>  &lt;PATH>-111-25-315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100000039&lt;/ACTION_ID>  &lt;PARENT_ID>1000010288&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CR REVIEW&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>applicantListFrmNavAction.do?actionPerformed=displayApplicantListInfo&amp;amp;screenId=1100000039&amp;amp;screenName=CR DECISION&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>CR REVIEW&lt;/MENU_TYPE>  &lt;SEQUENCE>5&lt;/SEQUENCE>  &lt;SCREEN_NAME>CREDIT REVIEW&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>persDescAction.do?actionPerformed=displayPersonalDiscussionPage&amp;amp;screenId=1000002044&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>5&lt;/SEQ_NO>  &lt;PATH>-111-25-35&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106515&lt;/ACTION_ID>  &lt;PARENT_ID/>  &lt;DISPLAY_NAME>GROUP&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>GROUP&lt;/MENU_TYPE>  &lt;SEQUENCE>111&lt;/SEQUENCE>  &lt;SCREEN_NAME>GROUP&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>newInsurance&lt;/REQUEST_PAGE>  &lt;SEQ_NO>111&lt;/SEQ_NO>  &lt;PATH>-1111&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106516&lt;/ACTION_ID>  &lt;PARENT_ID>1100106515&lt;/PARENT_ID>  &lt;DISPLAY_NAME>NEW GROUP&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>groupAction.do?actionPerformed=displayGroup&amp;amp;screenId=1100106516&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>GROUP&lt;/MENU_TYPE>  &lt;SEQUENCE>23&lt;/SEQUENCE>  &lt;SCREEN_NAME>NEW GROUP&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>23&lt;/SEQ_NO>  &lt;PATH>-1111-223&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106517&lt;/ACTION_ID>  &lt;PARENT_ID>1100106515&lt;/PARENT_ID>  &lt;DISPLAY_NAME>GROUP EDIT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>groupAction.do?actionPerformed=displayGroup&amp;amp;screenId=1100106517&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>GROUP&lt;/MENU_TYPE>  &lt;SEQUENCE>24&lt;/SEQUENCE>  &lt;SCREEN_NAME>GROUP EDIT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>24&lt;/SEQ_NO>  &lt;PATH>-1111-224&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200006000&lt;/ACTION_ID>  &lt;PARENT_ID/>  &lt;DISPLAY_NAME>LIMIT(LEASE)&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMITS&lt;/MENU_TYPE>  &lt;SEQUENCE>112&lt;/SEQUENCE>  &lt;SCREEN_NAME>LIMIT(LEASE)&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>112&lt;/SEQ_NO>  &lt;PATH>-1112&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200007001&lt;/ACTION_ID>  &lt;PARENT_ID>1200006000&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER LIMIT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMITS&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER LIMIT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-1112-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200006001&lt;/ACTION_ID>  &lt;PARENT_ID>1200006000&lt;/PARENT_ID>  &lt;DISPLAY_NAME>GROUP LIMIT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMITS&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>GROUP LIMIT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-1112-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200007002&lt;/ACTION_ID>  &lt;PARENT_ID>1200007001&lt;/PARENT_ID>  &lt;DISPLAY_NAME>NEW&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMITS&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>NEW&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>customerLimit.do?actionPerformed=displayNewCustomerLimit&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-1112-21-31&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200007037&lt;/ACTION_ID>  &lt;PARENT_ID>1200007002&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER LIMIT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>customerLimit.do?actionPerformed=displayNewCustomerLimit&amp;amp;screenId=1200007037&amp;amp;screenName=CUSTOMER LIMIT&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMITS&lt;/MENU_TYPE>  &lt;SEQUENCE>11&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER LIMIT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>customerLimit.do?actionPerformed=displayNewCustomerLimit&amp;amp;screenId=1200007037&amp;amp;screenName=CUSTOMER LIMIT&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>customerLimit.do?actionPerformed=displayNewCustomerLimit&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>11&lt;/SEQ_NO>  &lt;PATH>-1112-21-31-411&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200007003&lt;/ACTION_ID>  &lt;PARENT_ID>1200007001&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PENDING ACTIVITIES&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMITS&lt;/MENU_TYPE>  &lt;SEQUENCE>2&lt;/SEQUENCE>  &lt;SCREEN_NAME>PENDING ACTIVITIES&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>2&lt;/SEQ_NO>  &lt;PATH>-1112-21-32&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200007004&lt;/ACTION_ID>  &lt;PARENT_ID>1200007003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>EDIT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>customerLimit.do?actionPerformed=customerLimitPendingSearchList&amp;amp;screenId=1200007004&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMITS&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>EDIT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>customerLimit.do?actionPerformed=displayCustomerLimitEdit&amp;amp;screenId=1200007030&amp;amp;screenName=CUSTOMER LIMIT EDIT&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-1112-21-32-41&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200007005&lt;/ACTION_ID>  &lt;PARENT_ID>1200007003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CR DECISION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>customerLimit.do?actionPerformed=customerLimitPendingSearchList&amp;amp;screenId=1200007005&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMITS&lt;/MENU_TYPE>  &lt;SEQUENCE>2&lt;/SEQUENCE>  &lt;SCREEN_NAME>CR DECISION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>creditDecision.do?actionPerformed=displayCreditDecisionPage&amp;amp;screenId=1200007034&amp;amp;screenName=CR DECISION&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>2&lt;/SEQ_NO>  &lt;PATH>-1112-21-32-42&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200006006&lt;/ACTION_ID>  &lt;PARENT_ID>1200006001&lt;/PARENT_ID>  &lt;DISPLAY_NAME>GROUP LIMIT VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>groupLimitAction.do?actionPerformed=groupLimitPendingSearch&amp;amp;screenId=1200006006&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMITS&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>GROUP LIMIT VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>groupLimitAction.do?actionPerformed=displayGroupLimitEdit&amp;amp;screenId=1200006030&amp;amp;screenName=EDIT&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-1112-21-33&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200007006&lt;/ACTION_ID>  &lt;PARENT_ID>1200007001&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER LIMIT VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>customerLimit.do?actionPerformed=customerLimitPendingSearchList&amp;amp;screenId=1200007006&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMITS&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER LIMIT VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>customerLimit.do?actionPerformed=displayCustomerLimitEdit&amp;amp;screenId=1200007030&amp;amp;screenName=EDIT&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-1112-21-33&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003000&lt;/ACTION_ID>  &lt;PARENT_ID/>  &lt;DISPLAY_NAME>QUOTATION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>12&lt;/SEQUENCE>  &lt;SCREEN_NAME>QUOTATION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>12&lt;/SEQ_NO>  &lt;PATH>-112&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003001&lt;/ACTION_ID>  &lt;PARENT_ID>1200003000&lt;/PARENT_ID>  &lt;DISPLAY_NAME>NEW&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>NEW&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospetFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>newProspect&lt;/REQUEST_PAGE>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-112-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003002&lt;/ACTION_ID>  &lt;PARENT_ID>1200003001&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE PROPOSAL&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>newEnquiry.sprg?screenId=1200003002&amp;amp;screenName=NEW ENQUIRY&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>91&lt;/SEQUENCE>  &lt;SCREEN_NAME>NEW ENQUIRY&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>newEnquiry.sprg&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>newProspect&lt;/REQUEST_PAGE>  &lt;SEQ_NO>91&lt;/SEQ_NO>  &lt;PATH>-112-21-391&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003003&lt;/ACTION_ID>  &lt;PARENT_ID>1200003000&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PENDING ACTIVITIES&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>23&lt;/SEQUENCE>  &lt;SCREEN_NAME>PENDING ACTIVITES&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>23&lt;/SEQ_NO>  &lt;PATH>-112-223&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003005&lt;/ACTION_ID>  &lt;PARENT_ID>1200003003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>ASSET DETAILS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>quotationList.sprg?screenId=1200003005&amp;amp;screenName=ASSET&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>ASSET&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>assetDetails.sprg?screenId=1200003012&amp;amp;screenName=ASSET&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-112-223-31&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003006&lt;/ACTION_ID>  &lt;PARENT_ID>1200003003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>QUOTATION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>quotationList.sprg?screenId=1200003006&amp;amp;screenName=QUOTATION&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>11&lt;/SEQUENCE>  &lt;SCREEN_NAME>QUOTATION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>quotationDetails.sprg?screenId=1200003013&amp;amp;screenName=QUOTATION&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>11&lt;/SEQ_NO>  &lt;PATH>-112-223-311&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003007&lt;/ACTION_ID>  &lt;PARENT_ID>1200003003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>INSURANCE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>quotationList.sprg?screenId=1200003007&amp;amp;screenName=INSURANCE&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>4&lt;/SEQUENCE>  &lt;SCREEN_NAME>INSURANCE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>insuranceQuotationDetails.sprg?screenId=1200003014&amp;amp;screenName=INSURANCE&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>4&lt;/SEQ_NO>  &lt;PATH>-112-223-34&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003008&lt;/ACTION_ID>  &lt;PARENT_ID>1200003003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CASHFLOW&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>quotationList.sprg?screenId=1200003008&amp;amp;screenName=CASHFLOW&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>5&lt;/SEQUENCE>  &lt;SCREEN_NAME>CASHFLOW&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>quotationCashflowDetail.sprg?screenId=1200003015&amp;amp;screenName=CASHFLOW&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>5&lt;/SEQ_NO>  &lt;PATH>-112-223-35&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003021&lt;/ACTION_ID>  &lt;PARENT_ID>1200003003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>REGISTRATION DETAILS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>quotationList.sprg?screenId=1200003021&amp;amp;screenName=REGISTRATION DETAILS&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>6&lt;/SEQUENCE>  &lt;SCREEN_NAME>REGISTRATION DETAILS&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>quotationVroDetail.sprg?&amp;amp;screenId=1200003022&amp;amp;screenName=REGISTRATION DETAILS&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>6&lt;/SEQ_NO>  &lt;PATH>-112-223-36&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003009&lt;/ACTION_ID>  &lt;PARENT_ID>1200003003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>ACCEPTANCE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>quotationList.sprg?screenId=1200003009&amp;amp;screenName=ACCEPTANCE&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>7&lt;/SEQUENCE>  &lt;SCREEN_NAME>ACCEPTANCE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>acceptanceDetails.sprg?screenId=1200003016&amp;amp;screenName=ACCEPTANCE&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>7&lt;/SEQ_NO>  &lt;PATH>-112-223-37&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003010&lt;/ACTION_ID>  &lt;PARENT_ID>1200003003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>ACQUISITION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>quotationList.sprg?screenId=1200003010&amp;amp;screenName=ACQUISITION&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>8&lt;/SEQUENCE>  &lt;SCREEN_NAME>ACQUISITION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>acquisitionDetails.sprg?screenId=1200003017&amp;amp;screenName=ACQUISITION&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>8&lt;/SEQ_NO>  &lt;PATH>-112-223-38&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200003020&lt;/ACTION_ID>  &lt;PARENT_ID>1200003000&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>quotationList.sprg?screenId=1200003020&amp;amp;screenName=VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>24&lt;/SEQUENCE>  &lt;SCREEN_NAME>VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>quotationDetails.sprg?screenId=1200003013&amp;amp;screenName=QUOTATION&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>24&lt;/SEQ_NO>  &lt;PATH>-112-224&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004000&lt;/ACTION_ID>  &lt;PARENT_ID/>  &lt;DISPLAY_NAME>DM QUOTATION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>13&lt;/SEQUENCE>  &lt;SCREEN_NAME>DM_LOS_LEASE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>13&lt;/SEQ_NO>  &lt;PATH>-113&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004001&lt;/ACTION_ID>  &lt;PARENT_ID>1200004000&lt;/PARENT_ID>  &lt;DISPLAY_NAME>NEW &lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>NEW_DM_APPLICATION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-113-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004002&lt;/ACTION_ID>  &lt;PARENT_ID>1200004001&lt;/PARENT_ID>  &lt;DISPLAY_NAME>DM APPLICATION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>dMQuotationApplicantDetail.sprg?screenId=1200003002&amp;amp;screenName=NEW DM APPLICATION&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>2&lt;/SEQUENCE>  &lt;SCREEN_NAME>DM_APPLICATION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>dMQuotationApplicantDetail.sprg&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>newProspect&lt;/REQUEST_PAGE>  &lt;SEQ_NO>2&lt;/SEQ_NO>  &lt;PATH>-113-21-32&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004003&lt;/ACTION_ID>  &lt;PARENT_ID>1200004000&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PENDING ACTIVITIES&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>2&lt;/SEQUENCE>  &lt;SCREEN_NAME>PENDING ACTIVITES&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>2&lt;/SEQ_NO>  &lt;PATH>-113-22&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004005&lt;/ACTION_ID>  &lt;PARENT_ID>1200004003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>ASSET DETAILS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>dmProspectList.sprg?screenId=1200004005&amp;amp;screenName=ASSET&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>2&lt;/SEQUENCE>  &lt;SCREEN_NAME>ASSET&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>assetDetails.sprg?screenId=1200004019&amp;amp;screenName=ASSET&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>2&lt;/SEQ_NO>  &lt;PATH>-113-22-32&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004006&lt;/ACTION_ID>  &lt;PARENT_ID>1200004003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>QUOTATION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>dmProspectList.sprg?screenId=1200004006&amp;amp;screenName=QUOTATION&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>QUOTATION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>quotationDetails.sprg?screenId=1200004020&amp;amp;screenName=QUOTATION&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-113-22-33&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004008&lt;/ACTION_ID>  &lt;PARENT_ID>1200004003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CASHFLOW&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>dmProspectList.sprg?screenId=1200004008&amp;amp;screenName=CASHFLOW&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>5&lt;/SEQUENCE>  &lt;SCREEN_NAME>CASHFLOW&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>quotationCashflowDetail.sprg?screenId=1200004022&amp;amp;screenName=CASHFLOW&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>5&lt;/SEQ_NO>  &lt;PATH>-113-22-35&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004013&lt;/ACTION_ID>  &lt;PARENT_ID>1200004003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>DM OFFLINE ACTION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>dmProspectList.sprg?screenId=1200004013&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>6&lt;/SEQUENCE>  &lt;SCREEN_NAME>DM OFFLINE ACTION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>dmWorkflow.sprg?screenId=1200004016&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>6&lt;/SEQ_NO>  &lt;PATH>-113-22-36&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004009&lt;/ACTION_ID>  &lt;PARENT_ID>1200004003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>DM SANCTION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>dmProspectList.sprg?screenId=1200004009&amp;amp;screenName=DM SANCTION&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>6&lt;/SEQUENCE>  &lt;SCREEN_NAME>DM SANCTION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>creditDecision.do?actionPerformed=displayCreditDecisionPage&amp;amp;screenId=1000000086&amp;amp;screenName=DM SANCTION&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>6&lt;/SEQ_NO>  &lt;PATH>-113-22-36&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004010&lt;/ACTION_ID>  &lt;PARENT_ID>1200004003&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CAPITALISATION  MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>dmProspectList.sprg?screenId=1200004010&amp;amp;screenName=DISBURSAL_MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>7&lt;/SEQUENCE>  &lt;SCREEN_NAME>CAPITALISATION  MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>disbursalFrmNav.do?actionPerformed=displayDMDisbursalInfo&amp;amp;screenId=1200004017&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>7&lt;/SEQ_NO>  &lt;PATH>-113-22-37&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004023&lt;/ACTION_ID>  &lt;PARENT_ID>1200004000&lt;/PARENT_ID>  &lt;DISPLAY_NAME>DM CANCELLATION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LOAN CANCELATION&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>DM CANCELLATION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLMS.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-113-23&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004012&lt;/ACTION_ID>  &lt;PARENT_ID>1200004000&lt;/PARENT_ID>  &lt;DISPLAY_NAME>DM VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>dmProspectList.sprg?screenId=1200004012&amp;amp;screenName=VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>DM VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>quotationDetails.sprg?screenId=1200004020&amp;amp;screenName=QUOTATION&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-113-23&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004024&lt;/ACTION_ID>  &lt;PARENT_ID>1200004023&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>dmProspectList.sprg?screenId=1200004024&amp;amp;screenName=CANCELLATION MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LOAN CANCELATION&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>DM CANCELLATION MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>disbCancellationAction.do?actionPerformed=displayDmCancellation&amp;amp;screenId=1200004027&amp;amp;screenName=CANCELLATION MAKER&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-113-23-31&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004026&lt;/ACTION_ID>  &lt;PARENT_ID>1200004023&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>dmProspectList.sprg?screenId=1200004026&amp;amp;screenName=CANCELLATION VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LOAN CANCELATION&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>DM CANCELLATION VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>disbCancellationAction.do?actionPerformed=displayDmCancellation&amp;amp;screenId=1200004026&amp;amp;screenName=CANCELLATION VIEWER&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/los/prospectFlowFromLLMHome.do&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE>prospectFlow&lt;/REQUEST_PAGE>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-113-23-33&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004029&lt;/ACTION_ID>  &lt;PARENT_ID>1200004000&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PAYMENT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PAYMENT&lt;/MENU_TYPE>  &lt;SEQUENCE>4&lt;/SEQUENCE>  &lt;SCREEN_NAME>PAYMENT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>4&lt;/SEQ_NO>  &lt;PATH>-113-24&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004030&lt;/ACTION_ID>  &lt;PARENT_ID>1200004029&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>forwardFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1200004030&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PAYMENT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>PAYMENT MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>paymentAction.do?actionPerformed=displayPaymentScreen&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-113-24-31&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200004032&lt;/ACTION_ID>  &lt;PARENT_ID>1200004029&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>forwardFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1200004032&amp;amp;screenName=VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PAYMENT&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>PAYMENT VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>paymentAction.do?actionPerformed=displayPaymentScreen&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-113-24-33&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000017&lt;/ACTION_ID>  &lt;PARENT_ID/>  &lt;DISPLAY_NAME>LMS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LMS&lt;/MENU_TYPE>  &lt;SEQUENCE>14&lt;/SEQUENCE>  &lt;SCREEN_NAME>LMS&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>14&lt;/SEQ_NO>  &lt;PATH>-114&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106854&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>INVOICE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>INVOICE&lt;/MENU_TYPE>  &lt;SEQUENCE>31&lt;/SEQUENCE>  &lt;SCREEN_NAME>INVOICE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>31&lt;/SEQ_NO>  &lt;PATH>-114-231&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106856&lt;/ACTION_ID>  &lt;PARENT_ID>1200106854&lt;/PARENT_ID>  &lt;DISPLAY_NAME>INVOICE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>forwardFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1200106856&amp;amp;screenName=INVOICE&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>INVOICE&lt;/MENU_TYPE>  &lt;SEQUENCE>50&lt;/SEQUENCE>  &lt;SCREEN_NAME>INVOICE&lt;/SCREEN_NAME>  &lt;AUTHMODE/>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>invoiceDetails.sprg?screenId=1200106856&amp;amp;screenName=INVOICE&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>50&lt;/SEQ_NO>  &lt;PATH>-114-231-350&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106858&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>DM PDE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>DM PDE&lt;/MENU_TYPE>  &lt;SEQUENCE>32&lt;/SEQUENCE>  &lt;SCREEN_NAME>DM PDE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>32&lt;/SEQ_NO>  &lt;PATH>-114-232&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106859&lt;/ACTION_ID>  &lt;PARENT_ID>1200106858&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1200106859&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>DM PDE&lt;/MENU_TYPE>  &lt;SEQUENCE>33&lt;/SEQUENCE>  &lt;SCREEN_NAME>DM PDE MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>pdeApplicantList.do?actionPerformed=displayPDECustomerList&amp;amp;screenId=1000000090&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>33&lt;/SEQ_NO>  &lt;PATH>-114-232-333&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000034&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>RECEIPT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>OTC&lt;/MENU_TYPE>  &lt;SEQUENCE>42&lt;/SEQUENCE>  &lt;SCREEN_NAME>OTC&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>42&lt;/SEQ_NO>  &lt;PATH>-114-242&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000036&lt;/ACTION_ID>  &lt;PARENT_ID>1000000034&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000036&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>OTC&lt;/MENU_TYPE>  &lt;SEQUENCE>44&lt;/SEQUENCE>  &lt;SCREEN_NAME>RECEIPT MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>otcFrmNavAction.do?actionPerformed=displayOtcInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/lms/otcFrmNavAction.do?actionPerformed=displayOtcInfo&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>44&lt;/SEQ_NO>  &lt;PATH>-114-242-344&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000038&lt;/ACTION_ID>  &lt;PARENT_ID>1000000034&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PAYMENT HISTORY&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000038&amp;amp;screenName=VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>OTC&lt;/MENU_TYPE>  &lt;SEQUENCE>46&lt;/SEQUENCE>  &lt;SCREEN_NAME>PAYMENT HISTORY&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>viewerAction.do?actionPerformed=displayViewerInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>otcFrmNavAction.do?actionPerformed=displayOtcInfo&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>46&lt;/SEQ_NO>  &lt;PATH>-114-242-346&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000010034&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>RECEIPT CANCELLATION &lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>RECEIPT_CANCELLATION &lt;/MENU_TYPE>  &lt;SEQUENCE>44&lt;/SEQUENCE>  &lt;SCREEN_NAME>RECEIPT_CANCELLATION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>44&lt;/SEQ_NO>  &lt;PATH>-114-244&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000010035&lt;/ACTION_ID>  &lt;PARENT_ID>1000010034&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000010035&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>RECEIPT_CANCELLATION &lt;/MENU_TYPE>  &lt;SEQUENCE>44&lt;/SEQUENCE>  &lt;SCREEN_NAME>RECEIPT CANCELLATION MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>receipt_cancellationFrmNavAction.do?actionPerformed=displayreceiptInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/lms/otcFrmNavAction.do?actionPerformed=displayOtcInfo&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>44&lt;/SEQ_NO>  &lt;PATH>-114-244-344&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000010109&lt;/ACTION_ID>  &lt;PARENT_ID>1000010034&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000010109&amp;amp;screenName=VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>RECEIPT CANCELLATION&lt;/MENU_TYPE>  &lt;SEQUENCE>46&lt;/SEQUENCE>  &lt;SCREEN_NAME>RECEIPT CANCELLATION VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>viewerAction.do?actionPerformed=displayViewerInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>otcFrmNavAction.do?actionPerformed=displayOtcInfo&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>46&lt;/SEQ_NO>  &lt;PATH>-114-244-346&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000039&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>REFUND&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REFUND&lt;/MENU_TYPE>  &lt;SEQUENCE>47&lt;/SEQUENCE>  &lt;SCREEN_NAME>REFUND&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>47&lt;/SEQ_NO>  &lt;PATH>-114-247&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000041&lt;/ACTION_ID>  &lt;PARENT_ID>1000000039&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000041&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REFUND&lt;/MENU_TYPE>  &lt;SEQUENCE>49&lt;/SEQUENCE>  &lt;SCREEN_NAME>REFUND MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>refundFrmNavAction.do?actionPerformed=displayRefundInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>49&lt;/SEQ_NO>  &lt;PATH>-114-247-349&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000043&lt;/ACTION_ID>  &lt;PARENT_ID>1000000039&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000043&amp;amp;screenName=VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REFUND&lt;/MENU_TYPE>  &lt;SEQUENCE>51&lt;/SEQUENCE>  &lt;SCREEN_NAME>REFUND VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>viewerAction.do?actionPerformed=displayViewerInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>51&lt;/SEQ_NO>  &lt;PATH>-114-247-351&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001980&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MANUAL ADVICE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>MANUAL ADVICE&lt;/MENU_TYPE>  &lt;SEQUENCE>48&lt;/SEQUENCE>  &lt;SCREEN_NAME>MANUAL ADVICE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>48&lt;/SEQ_NO>  &lt;PATH>-114-248&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001981&lt;/ACTION_ID>  &lt;PARENT_ID>1000001980&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000001981&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>MANUAL ADVICE&lt;/MENU_TYPE>  &lt;SEQUENCE>10&lt;/SEQUENCE>  &lt;SCREEN_NAME>MANUAL ADVICE MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>manualAdviceAction.do?actionPerformed=displayManualAdviceInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/lms/manualAdviceFrmNavAction.do?actionPerformed=displayManualAdviceInfo&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>10&lt;/SEQ_NO>  &lt;PATH>-114-248-310&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000054&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>KNOCK OFF&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>KNOCK OFF&lt;/MENU_TYPE>  &lt;SEQUENCE>57&lt;/SEQUENCE>  &lt;SCREEN_NAME>KNOCK OFF&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>57&lt;/SEQ_NO>  &lt;PATH>-114-257&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000056&lt;/ACTION_ID>  &lt;PARENT_ID>1000000054&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000056&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>KNOCK OFF&lt;/MENU_TYPE>  &lt;SEQUENCE>59&lt;/SEQUENCE>  &lt;SCREEN_NAME>KNOCK OFF MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>knockOffFrmNavAction.do?actionPerformed=displayKnockOffInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>59&lt;/SEQ_NO>  &lt;PATH>-114-257-359&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000058&lt;/ACTION_ID>  &lt;PARENT_ID>1000000054&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000058&amp;amp;screenName=VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>KNOCK OFF&lt;/MENU_TYPE>  &lt;SEQUENCE>61&lt;/SEQUENCE>  &lt;SCREEN_NAME>KNOCK OFF VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>viewerAction.do?actionPerformed=displayViewerInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>knockOffFrmNavAction.do?actionPerformed=displayKnockOffInfo&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>61&lt;/SEQ_NO>  &lt;PATH>-114-257-361&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000024&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>INSTR MANAGEMENT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>INSTRUMENT MANAGEMENT&lt;/MENU_TYPE>  &lt;SEQUENCE>59&lt;/SEQUENCE>  &lt;SCREEN_NAME>INSTRUMENT MANAGEMENT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>59&lt;/SEQ_NO>  &lt;PATH>-114-259&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000025&lt;/ACTION_ID>  &lt;PARENT_ID>1000000024&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PDC/ECS GENERATE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3_1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PDC_GENERATE&lt;/MENU_TYPE>  &lt;SEQUENCE>33&lt;/SEQUENCE>  &lt;SCREEN_NAME>PDC/ECS GENERATE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>33&lt;/SEQ_NO>  &lt;PATH>-114-259-333&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000026&lt;/ACTION_ID>  &lt;PARENT_ID>1000000025&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000026&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PDC_GENERATE&lt;/MENU_TYPE>  &lt;SEQUENCE>35&lt;/SEQUENCE>  &lt;SCREEN_NAME>PDC/ECS GENERATE MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>pdcEcsFrmNavAction.do?actionPerformed=displayPdcEcsInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/lms/pdcEcsFrmNavAction.do?actionPerformed=displayPdcEcsInfo&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>35&lt;/SEQ_NO>  &lt;PATH>-114-259-333-435&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000029&lt;/ACTION_ID>  &lt;PARENT_ID>1000000024&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PDC/ECS EDIT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>Y&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3_1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PDC_EDIT&lt;/MENU_TYPE>  &lt;SEQUENCE>37&lt;/SEQUENCE>  &lt;SCREEN_NAME>PDC/ECS EDIT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>37&lt;/SEQ_NO>  &lt;PATH>-114-259-337&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000031&lt;/ACTION_ID>  &lt;PARENT_ID>1000000029&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000031&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PDC_EDIT&lt;/MENU_TYPE>  &lt;SEQUENCE>39&lt;/SEQUENCE>  &lt;SCREEN_NAME>PDC/ECS EDIT MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>pdcEcsEditFrmNavAction.do?actionPerformed=displayPdcEcsEditInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>http://xxx.xxx.xxx.xxx:xxxx/lms/pdcEcsEditFrmNavAction.do?actionPerformed=displayPdcEcsEditInfo&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>39&lt;/SEQ_NO>  &lt;PATH>-114-259-337-439&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000033&lt;/ACTION_ID>  &lt;PARENT_ID>1000000029&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000033&amp;amp;screenName=VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PDC_EDIT&lt;/MENU_TYPE>  &lt;SEQUENCE>41&lt;/SEQUENCE>  &lt;SCREEN_NAME>PDC/ECS EDIT VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>viewerAction.do?actionPerformed=displayViewerInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>pdcEcsEditFrmNavAction.do?actionPerformed=displayPdcEcsEditInfo&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>41&lt;/SEQ_NO>  &lt;PATH>-114-259-337-441&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000049&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>WAIVE OFF&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>WAIVEOFF&lt;/MENU_TYPE>  &lt;SEQUENCE>60&lt;/SEQUENCE>  &lt;SCREEN_NAME>WAIVEOFF&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>60&lt;/SEQ_NO>  &lt;PATH>-114-260&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000051&lt;/ACTION_ID>  &lt;PARENT_ID>1000000049&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000051&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>WAIVE&lt;/MENU_TYPE>  &lt;SEQUENCE>54&lt;/SEQUENCE>  &lt;SCREEN_NAME>WAIVE MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>waiveOffFrmNavAction.do?actionPerformed=displayWaiveOffInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>54&lt;/SEQ_NO>  &lt;PATH>-114-260-354&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000053&lt;/ACTION_ID>  &lt;PARENT_ID>1000000049&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000053&amp;amp;screenName=VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>WAIVE&lt;/MENU_TYPE>  &lt;SEQUENCE>56&lt;/SEQUENCE>  &lt;SCREEN_NAME>WAIVE VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>viewerAction.do?actionPerformed=displayViewerInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL>waiveOffFrmNavAction.do?actionPerformed=displayWaiveOffInfo&lt;/APPLICATION_URL>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>56&lt;/SEQ_NO>  &lt;PATH>-114-260-356&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000059&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CLOSE / FORECLOSE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>FORECLOSURE&lt;/MENU_TYPE>  &lt;SEQUENCE>62&lt;/SEQUENCE>  &lt;SCREEN_NAME>CLOSE/FORECLOSE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>62&lt;/SEQ_NO>  &lt;PATH>-114-262&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000061&lt;/ACTION_ID>  &lt;PARENT_ID>1000000059&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000061&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>FORECLOSURE&lt;/MENU_TYPE>  &lt;SEQUENCE>64&lt;/SEQUENCE>  &lt;SCREEN_NAME>CLOSE/FORECLOSE MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>foreClouserFrmNavAction.do?actionPerformed=displayForeClouserInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>64&lt;/SEQ_NO>  &lt;PATH>-114-262-364&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000063&lt;/ACTION_ID>  &lt;PARENT_ID>1000000059&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000063&amp;amp;screenName=VIEWER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>FORECLOSURE&lt;/MENU_TYPE>  &lt;SEQUENCE>66&lt;/SEQUENCE>  &lt;SCREEN_NAME>CLOSE/FORECLOSE VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>V&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>foreClouserFrmNavAction.do?actionPerformed=displayForeClouserInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>Y&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>66&lt;/SEQ_NO>  &lt;PATH>-114-262-366&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000064&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BATCH PROCESS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH PROCESS&lt;/MENU_TYPE>  &lt;SEQUENCE>67&lt;/SEQUENCE>  &lt;SCREEN_NAME>BATCH PROCESS&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>67&lt;/SEQ_NO>  &lt;PATH>-114-267&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000065&lt;/ACTION_ID>  &lt;PARENT_ID>1000000064&lt;/PARENT_ID>  &lt;DISPLAY_NAME>GENERATION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>forwardFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000000065&amp;amp;screenName=GENERATION&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3_1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_GENERATION&lt;/MENU_TYPE>  &lt;SEQUENCE>68&lt;/SEQUENCE>  &lt;SCREEN_NAME>GENERATION&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>batchGenerationFrmNavAction.do?actionPerformed=displayBatchProcessInfo&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>68&lt;/SEQ_NO>  &lt;PATH>-114-267-368&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001045&lt;/ACTION_ID>  &lt;PARENT_ID>1000000017&lt;/PARENT_ID>  &lt;DISPLAY_NAME>RESCHEDULING CASE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>RESCHEDULE  CASE&lt;/MENU_TYPE>  &lt;SEQUENCE>82&lt;/SEQUENCE>  &lt;SCREEN_NAME>RESCHEDULE CASE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>82&lt;/SEQ_NO>  &lt;PATH>-114-282&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001046&lt;/ACTION_ID>  &lt;PARENT_ID>1000001045&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>prospectListFrmNavAction.do?actionPerformed=displayProspectListInfo&amp;amp;screenId=1000001046&amp;amp;screenName=MAKER&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>RESCHEDULE  CASE&lt;/MENU_TYPE>  &lt;SEQUENCE>83&lt;/SEQUENCE>  &lt;SCREEN_NAME>RESCHEDULE CASE MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>rescheduleCaseFrmNavAction.do?actionPerformed=displayRescheduleCaseData&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>83&lt;/SEQ_NO>  &lt;PATH>-114-282-383&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000000074&lt;/ACTION_ID>  &lt;PARENT_ID/>  &lt;DISPLAY_NAME>REPORTS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>17&lt;/SEQUENCE>  &lt;SCREEN_NAME>REPORTS&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>17&lt;/SEQ_NO>  &lt;PATH>-117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000010249&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>ACCOUNTING REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1000010249&amp;amp;actionId=1000010249&amp;amp;mode=R&amp;amp;actionName=ACCOUNTING_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>ACCOUNTING_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>ACCOUNTING_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1000010249\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109018&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>ACCRUAL REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109018&amp;amp;actionId=1200109018&amp;amp;mode=R&amp;amp;actionName=ACCRUAL_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>ACCRUAL_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>ACCRUAL REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109018\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109019&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>ASSET RECEIPT REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109019&amp;amp;actionId=1200109019&amp;amp;mode=R&amp;amp;actionName=ASSET_RECEIPT_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>ASSET_RECEIPT_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>ASSET RECEIPT REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109019\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106909&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>AUTO RESCHEDULE STATUS REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1100106909&amp;amp;actionId=1100106909&amp;amp;mode=R&amp;amp;actionName=AUTO_RESCHEDULE_STATUS_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>AUTO_RESCHEDULE_STATUS_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>AUTO_RESCHEDULE_STATUS_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1100106909\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106908&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BASE_RATE_CHANGE_TRACK_REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1100106908&amp;amp;actionId=1100106908&amp;amp;mode=R&amp;amp;actionName=BASE_RATE_CHANGE_TRACK_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BASE_RATE_CHANGE_TRACK_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>BASE_RATE_CHANGE_TRACK_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1100106908\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109049&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CLIENT MIS REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109049&amp;amp;actionId=1200109049&amp;amp;mode=R&amp;amp;actionName=CLIENT_MIS_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>CLIENT MIS REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109049\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109103&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER DETAIL REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109103&amp;amp;actionId=1200109103&amp;amp;mode=R&amp;amp;actionName=CUSTOMER_DETAIL_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>CUSTOMER DETAIL REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER DETAIL REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109103\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109098&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>DOCUMENT PENDING AND EXPIRY REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109098&amp;amp;actionId=1200109098&amp;amp;mode=R&amp;amp;actionName=DOCUMENT_EXPIRY&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>DOCUMENT EXPIRY&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>DOCUMENT PENDING AND EXPIRY REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109097\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109124&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>INSURANCE REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109124&amp;amp;actionId=1200109124&amp;amp;mode=R&amp;amp;actionName=INSURANCE_EXPIRY_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>INSURANCE REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109124\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109010&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>INVOICE REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109010&amp;amp;actionId=1200109010&amp;amp;mode=R&amp;amp;actionName=INVOICE_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>INVOICE_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>INVOICE REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109010\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109090&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEAD FOLLOW UP&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109090&amp;amp;actionId=1200109090&amp;amp;mode=R&amp;amp;actionName=LEAD_FOLLOW_UP&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LEAD FOLLOW UP&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEAD FOLLOW UP&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109090\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109037&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEAD STATUS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109037&amp;amp;actionId=1200109037&amp;amp;mode=R&amp;amp;actionName=LEAD_STATUS&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LEAD STATUS&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEAD STATUS&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109037\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109169&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE REGISTRATION TRACKING&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109169&amp;amp;actionId=1200109169&amp;amp;mode=R&amp;amp;actionName=LEASE_REGISTRATION_TRACKING&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LEASE REGISTRATION TRACKING&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE REGISTRATION TRACKING&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109169\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109097&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LIMIT EXPIRY&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109097&amp;amp;actionId=1200109097&amp;amp;mode=R&amp;amp;actionName=LIMIT_EXPIRY&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LIMIT EXPIRY&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>LIMIT EXPIRY&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109097\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109128&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LPI REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109128&amp;amp;actionId=1200109128&amp;amp;mode=R&amp;amp;actionName=LPI REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>LPI REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109128\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109170&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MATURING AGREEMENTS REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109170&amp;amp;actionId=1200109170&amp;amp;mode=R&amp;amp;actionName=MATURING_AGREEMENTS_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>MATURING AGREEMENTS REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>MATURING AGREEMENTS&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109170\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109030&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>OUTSTANDING DUE REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109030&amp;amp;actionId=1200109030&amp;amp;mode=R&amp;amp;actionName=OUTSTANDING_DUE_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>OUTSTANDING_DUE_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>OUTSTANDING DUE REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109030\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109029&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>RV DUE REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109029&amp;amp;actionId=1200109029&amp;amp;mode=R&amp;amp;actionName=RV_DUE_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>RV_DUE_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>RV DUE REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109029\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109062&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>SCHEDULED REPORTS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109062&amp;amp;actionId=1200109062&amp;amp;mode=R&amp;amp;actionName=SCHEDULED_REPORTS&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>SCHEDULED REPORTS&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109062\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109017&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>TRANSACTION REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109017&amp;amp;actionId=1200109017&amp;amp;mode=R&amp;amp;actionName=TRANSACTION_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>TRANSACTION_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>TRANSACTION REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109017\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109125&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>WAIVER_REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109125&amp;amp;actionId=1200109125&amp;amp;mode=R&amp;amp;actionName=WAIVER_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>WAIVER_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109125\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-117-21&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108937&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MANUAL ADVICE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108937&amp;amp;actionId=1200108937&amp;amp;mode=R&amp;amp;actionName=MANUAL_ADVICE&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>MANUAL_ADVICE&lt;/MENU_TYPE>  &lt;SEQUENCE>105&lt;/SEQUENCE>  &lt;SCREEN_NAME>MANUAL_ADVICE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200108937\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>105&lt;/SEQ_NO>  &lt;PATH>-117-2105&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109133&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE PROPOSAL&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109133&amp;amp;actionId=1200109133&amp;amp;mode=R&amp;amp;actionName=LEASE_PROPOSAL&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_PROPOSAL&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109133&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-2117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106732&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PO REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200106732&amp;amp;actionId=1200106732&amp;amp;mode=R&amp;amp;actionName=PO_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PO_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>PO_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200106732&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-2117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108995&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>QUOTATION DETAIL REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108995&amp;amp;actionId=1200108995&amp;amp;mode=R&amp;amp;actionName=QUOTATION_DETAIL_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QUOTATION_DETAIL_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>121&lt;/SEQUENCE>  &lt;SCREEN_NAME>QUOTATION DETAIL REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200108995\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>121&lt;/SEQ_NO>  &lt;PATH>-117-2121&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100010146&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>RESCHEDULING REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1100010146&amp;amp;actionId=1100010146&amp;amp;mode=R&amp;amp;actionName=RESCHEDULING_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>RESCHEDULING_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>129&lt;/SEQUENCE>  &lt;SCREEN_NAME>RESCHEDULING_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1100010146\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>129&lt;/SEQ_NO>  &lt;PATH>-117-2129&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106857&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BATCH DTL REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200106857&amp;amp;actionId=1200106857&amp;amp;mode=R&amp;amp;actionName=BATCH_DTL_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_DTL_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>13&lt;/SEQUENCE>  &lt;SCREEN_NAME>BATCH_DTL_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200106857\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>13&lt;/SEQ_NO>  &lt;PATH>-117-213&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106489&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>STATEMENT OF ACCOUNT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1100106489&amp;amp;actionId=1100106489&amp;amp;mode=R&amp;amp;actionName=SOA_NEW&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>137&lt;/SEQUENCE>  &lt;SCREEN_NAME>STATEMENT_OF_ACCOUNT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1100106489\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>137&lt;/SEQ_NO>  &lt;PATH>-117-2137&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106727&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BATCH PRESENTATION REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1100106727&amp;amp;actionId=1100106727&amp;amp;mode=R&amp;amp;actionName=BATCH_PRESENTATION_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_PRESENTATION_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>14&lt;/SEQUENCE>  &lt;SCREEN_NAME>BATCH_PRESENTATION_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1100106727\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>14&lt;/SEQ_NO>  &lt;PATH>-117-214&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106510&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>TRANSACTION REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1100106510&amp;amp;actionId=1100106510&amp;amp;mode=R&amp;amp;actionName=TRANSACTION_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>TRANSACTION_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>140&lt;/SEQUENCE>  &lt;SCREEN_NAME>TRANSACTION REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1100106510\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>140&lt;/SEQ_NO>  &lt;PATH>-117-2140&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106511&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>TRIAL BALANCE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1100106511&amp;amp;actionId=1100106511&amp;amp;mode=R&amp;amp;actionName=TRIAL_BALANCE&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>TRIAL_BALANCE&lt;/MENU_TYPE>  &lt;SEQUENCE>141&lt;/SEQUENCE>  &lt;SCREEN_NAME>TRIAL BALANCE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1100106511\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>141&lt;/SEQ_NO>  &lt;PATH>-117-2141&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108988&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VENDOR PAYMENT REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108988&amp;amp;actionId=1200108988&amp;amp;mode=R&amp;amp;actionName=VENDOR_PAYMENT_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>VENDOR_PAYMENT_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>145&lt;/SEQUENCE>  &lt;SCREEN_NAME>VENDOR PAYMENT REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200108988\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>145&lt;/SEQ_NO>  &lt;PATH>-117-2145&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106729&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BATCH STATUS REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1100106729&amp;amp;actionId=1100106729&amp;amp;mode=R&amp;amp;actionName=BATCH_STATUS_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_STATUS_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>15&lt;/SEQUENCE>  &lt;SCREEN_NAME>BATCH_STATUS_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1100106729\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>15&lt;/SEQ_NO>  &lt;PATH>-117-215&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108922&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BATCH UPLOAD ISSUES REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108922&amp;amp;actionId=1200108922&amp;amp;mode=R&amp;amp;actionName=BATCH_UPLOAD_ISSUES_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD_ISSUES_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>16&lt;/SEQUENCE>  &lt;SCREEN_NAME>BATCH_UPLOAD_ISSUES_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;=1200108922&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>16&lt;/SEQ_NO>  &lt;PATH>-117-216&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109108&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CREDIT OPERATION REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>17&lt;/SEQUENCE>  &lt;SCREEN_NAME>CREDIT_OPERATION_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>17&lt;/SEQ_NO>  &lt;PATH>-117-217&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109132&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>ANNEXES OL&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109132&amp;amp;actionId=1200109132&amp;amp;mode=R&amp;amp;actionName=ANNEXES_OL&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>ANNEXES_OL&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109132&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109137&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>AUDIT CONFIRMATION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109137&amp;amp;actionId=1200109137&amp;amp;mode=R&amp;amp;actionName=AUDIT_CONFIRMATION&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>AUDIT_CONFIRMATION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109137&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109121&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BOARD OF RESOLUTION&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109121&amp;amp;actionId=1200109121&amp;amp;mode=R&amp;amp;actionName=BOARD_OF_RESOLUTION&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>BOARD_OF_RESOLUTION&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109121&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109112&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BOARD RESOLUTION CORPORATE GUARANTOR&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109112&amp;amp;actionId=1200109112&amp;amp;mode=R&amp;amp;actionName=BOARD_RESOLUTION_CORPORATE_GUARANTOR&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>BOARD_RESOLUTION_CORPORATE_GUARANTOR&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109112&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109136&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CANCELLATION SALARY DEDUCTION FORM&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109136&amp;amp;actionId=1200109136&amp;amp;mode=R&amp;amp;actionName=CANCELLATION_SALARY_DEDUCTION_FORM&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>CANCELLATION_SALARY_DEDUCTION_FORM&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109136&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109135&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CANCELLATION STANDING ORDER FORM&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109135&amp;amp;actionId=1200109135&amp;amp;mode=R&amp;amp;actionName=CANCELLATION_STANDING_ORDER_FORM&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>CANCELLATION_STANDING_ORDER_FORM&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109135&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109119&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CERTIFICAT DE GAGE SECOND HAND&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109119&amp;amp;actionId=1200109119&amp;amp;mode=R&amp;amp;actionName=CERTIFICAT_DE_GAGE_SECOND_HAND&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>CERTIFICAT_DE_GAGE_SECOND_HAND&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109119&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109113&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CHANGE IN ENGINE NUMBER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109113&amp;amp;actionId=1200109113&amp;amp;mode=R&amp;amp;actionName=CHANGE_IN_ENGINE_NUMBER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>CHANGE_IN_ENGINE_NUMBER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109113&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109114&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CHANGE IN REGISTRATION NUMBER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109114&amp;amp;actionId=1200109114&amp;amp;mode=R&amp;amp;actionName=CHANGE_IN_REGISTRATION_NUMBER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>CHANGE_IN_REGISTRATION_NUMBER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109114&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109115&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CHANGE OF NAME OF HORSEPOWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109115&amp;amp;actionId=1200109115&amp;amp;mode=R&amp;amp;actionName=CHANGE_OF_NAME_OF_HORSEPOWER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>CHANGE_OF_NAME_OF_HORSEPOWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109115&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109116&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CORPORATE GUARANTEE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109116&amp;amp;actionId=1200109116&amp;amp;mode=R&amp;amp;actionName=CORPORATE_GUARANTEE&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>CORPORATE_GUARANTEE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109116&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109155&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>COVER LETTER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109155&amp;amp;actionId=1200109155&amp;amp;mode=R&amp;amp;actionName=COVER_LETTER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>COVER_LETTER&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109155&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109126&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>DIRECT DEBIT FORM&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109126&amp;amp;actionId=1200109126&amp;amp;mode=R&amp;amp;actionName=DIRECT_DEBIT_FORM&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>DIRECT_DEBIT_FORM&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109126&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109117&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>HORSEPOWER FULL&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109117&amp;amp;actionId=1200109117&amp;amp;mode=R&amp;amp;actionName=HORSEPOWER_FULL&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>HORSEPOWER_FULL&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109117&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109143&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE AGREEMENT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109143&amp;amp;actionId=1200109143&amp;amp;mode=R&amp;amp;actionName=LEASE_AGREEMENT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_AGREEMENT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109143&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109131&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE AGREEMENT OL&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109131&amp;amp;actionId=1200109131&amp;amp;mode=R&amp;amp;actionName=LEASE_AGREEMENT_OL&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_AGREEMENT_OL&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109131&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109111&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LETTER OF UNDERTAKING&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109111&amp;amp;actionId=1200109111&amp;amp;mode=R&amp;amp;actionName=LETTER_OF_UNDERTAKING&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>LETTER_OF_UNDERTAKING&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109111&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109118&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LOST HORSEPOWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109118&amp;amp;actionId=1200109118&amp;amp;mode=R&amp;amp;actionName=LOST_HORSEPOWER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>LOST_HORSEPOWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109118&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109120&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>NO LIABILITY LETTER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109120&amp;amp;actionId=1200109120&amp;amp;mode=R&amp;amp;actionName=NO_LIABILITY_LETTER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>NO_LIABILITY_LETTER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109120&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109141&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>OFFER LETTER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109141&amp;amp;actionId=1200109141&amp;amp;mode=R&amp;amp;actionName=OFFER_LETTER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>OFFER_LETTER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109141&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109067&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>REPAYMENT SCHEDULE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109067&amp;amp;actionId=1200109067&amp;amp;mode=R&amp;amp;actionName=REPAYMENT_SCHEDULE&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPAYMENT_SCHEDULE&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>REPAYMENT_SCHEDULE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109067&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109109&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>RIGHT OF SET OFF LETTER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109109&amp;amp;actionId=1200109109&amp;amp;mode=R&amp;amp;actionName=RIGHT_OF_SET_OFF_LETTER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>RIGHT_OF_SET_OFF_LETTER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109109&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109140&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>RV LETTER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109140&amp;amp;actionId=1200109140&amp;amp;mode=R&amp;amp;actionName=RV_LETTER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>RV_LETTER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109140&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109139&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>RV SALES DEED AND CERTIFICATE OF GAGE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109139&amp;amp;actionId=1200109139&amp;amp;mode=R&amp;amp;actionName=RV_SALES_DEED_AND_CERTIFICATE_OF_GAGE&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>RV_SALES_DEED_AND_CERTIFICATE_OF_GAGE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109139&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109134&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>SALARY DEDUCTION FORM&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109134&amp;amp;actionId=1200109134&amp;amp;mode=R&amp;amp;actionName=SALARY_DEDUCTION_FORM&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>SALARY_DEDUCTION_FORM&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109134&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109138&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>SALES DEED&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109138&amp;amp;actionId=1200109138&amp;amp;mode=R&amp;amp;actionName=SALES_DEED&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>SALES_DEED&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109138&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109003&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>SETTLEMENT LETTER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109003&amp;amp;actionId=1200109003&amp;amp;mode=R&amp;amp;actionName=FORECLOSURE_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>FORECLOSURE_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109003&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109127&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>STANDING ORDER FORM&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109127&amp;amp;actionId=1200109127&amp;amp;mode=R&amp;amp;actionName=STANDING_ORDER_FORM&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>STANDING_ORDER_FORM&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109127&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109110&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>SUBORDINATION OF SHAREHOLDERS LOAN&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109110&amp;amp;actionId=1200109110&amp;amp;mode=R&amp;amp;actionName=SUBORDINATION_OF_SHAREHOLDERS_LOAN&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>SUBORDINATION_OF_SHAREHOLDERS_LOAN&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109110&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109142&lt;/ACTION_ID>  &lt;PARENT_ID>1200109108&lt;/PARENT_ID>  &lt;DISPLAY_NAME>SUPPLIER LETTER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109142&amp;amp;actionId=1200109142&amp;amp;mode=R&amp;amp;actionName=SUPPLIER_LETTER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORTS&lt;/MENU_TYPE>  &lt;SEQUENCE>117&lt;/SEQUENCE>  &lt;SCREEN_NAME>SUPPLIER_LETTER&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109142&amp;amp;viewMode=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>117&lt;/SEQ_NO>  &lt;PATH>-117-217-3117&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108947&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BULK INVOICE REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>llmDashboard.do?actionPerformed=getDashboard&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BULK_INVOICE_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>20&lt;/SEQUENCE>  &lt;SCREEN_NAME>BULK_INVOICE_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>llmDashboard.do?actionPerformed=getDashboard&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>20&lt;/SEQ_NO>  &lt;PATH>-117-220&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106855&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>COVERNOTE REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>llmDashboard.do?actionPerformed=getDashboard&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>COVERNOTE_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>37&lt;/SEQUENCE>  &lt;SCREEN_NAME>COVERNOTE_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>llmDashboard.do?actionPerformed=getDashboard&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>37&lt;/SEQ_NO>  &lt;PATH>-117-237&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106539&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>AGEING REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1100106539&amp;amp;actionId=1100106539&amp;amp;mode=R&amp;amp;actionName=AGING_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>AGEING_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>4&lt;/SEQUENCE>  &lt;SCREEN_NAME>AGEING REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1100106539\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>4&lt;/SEQ_NO>  &lt;PATH>-117-24&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108999&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BRANCH WISE SUMMARY&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108999&amp;amp;actionId=1200108999&amp;amp;mode=R&amp;amp;actionName=BRANCH_WISE&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>40&lt;/SEQUENCE>  &lt;SCREEN_NAME>BRANCH WISE SUMMARY&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200108999\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>40&lt;/SEQ_NO>  &lt;PATH>-117-240&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109000&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>USER WISE SUMMARY&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200109000&amp;amp;actionId=1200109000&amp;amp;mode=R&amp;amp;actionName=USER_WISE&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>40&lt;/SEQUENCE>  &lt;SCREEN_NAME>USER WISE SUMMARY&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200109000\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>40&lt;/SEQ_NO>  &lt;PATH>-117-240&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108980&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER LIMIT REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108980&amp;amp;actionId=1200108980&amp;amp;mode=R&amp;amp;actionName=CUSTOMER_LIMIT_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>CUSTOMER_LIMIT_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>42&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER LIMIT REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200108980\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>42&lt;/SEQ_NO>  &lt;PATH>-117-242&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108996&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>FORECLOSURE REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108996&amp;amp;actionId=1200108996&amp;amp;mode=R&amp;amp;actionName=FORECLOSURE_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>FORECLOSURE_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>57&lt;/SEQUENCE>  &lt;SCREEN_NAME>FORECLOSURE REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200108996\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>57&lt;/SEQ_NO>  &lt;PATH>-117-257&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106710&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>ALCO REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1100106710&amp;amp;actionId=1100106710&amp;amp;mode=R&amp;amp;actionName=ALCO_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>ALCO_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>6&lt;/SEQUENCE>  &lt;SCREEN_NAME>ALCO_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1100106710\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>6&lt;/SEQ_NO>  &lt;PATH>-117-26&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108979&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>GROUP CUSTOMER REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108979&amp;amp;actionId=1200108979&amp;amp;mode=R&amp;amp;actionName=GROUP_CUSTOMER_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>GROUP_CUSTOMER_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>61&lt;/SEQUENCE>  &lt;SCREEN_NAME>GROUP CUSTOMER REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200108979\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>61&lt;/SEQ_NO>  &lt;PATH>-117-261&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108981&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>GROUP LIMIT REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108981&amp;amp;actionId=1200108981&amp;amp;mode=R&amp;amp;actionName=GROUP_LIMIT_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>GROUP_LIMIT_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>62&lt;/SEQUENCE>  &lt;SCREEN_NAME>GROUP LIMIT REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200108981\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>62&lt;/SEQ_NO>  &lt;PATH>-117-262&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108929&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE CAM REPORT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108929&amp;amp;actionId=1200108929&amp;amp;mode=R&amp;amp;actionName=LEASE_CAM_REPORT&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LEASE_CAM_REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>90&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_CAM_REPORT&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;=1200108929&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>90&lt;/SEQ_NO>  &lt;PATH>-117-290&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108906&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE CREDIT NOTE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108906&amp;amp;actionId=1200108906&amp;amp;mode=R&amp;amp;actionName=LEASE_CREDIT_NOTE&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>LEASE_CREDIT_NOTE&lt;/MENU_TYPE>  &lt;SEQUENCE>92&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_CREDIT_NOTE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl&amp;amp;=1200108906&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>92&lt;/SEQ_NO>  &lt;PATH>-117-292&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108876&lt;/ACTION_ID>  &lt;PARENT_ID>1000000074&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VAT INVOICE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>grfAction.do?actionPerformed=searchControl&amp;amp;screenId=1200108876&amp;amp;actionId=1200108876&amp;amp;mode=R&amp;amp;actionName=LEASE_SALE_INVOICE&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>REPORT&lt;/MENU_TYPE>  &lt;SEQUENCE>98&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_SALE_INVOICE&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>grfAction.do?actionPerformed=searchControl\&amp;amp;=1200108876\&amp;amp;=R&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>98&lt;/SEQ_NO>  &lt;PATH>-117-298&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001024&lt;/ACTION_ID>  &lt;PARENT_ID/>  &lt;DISPLAY_NAME>MASTERS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>MASTER&lt;/MENU_TYPE>  &lt;SEQUENCE>18&lt;/SEQUENCE>  &lt;SCREEN_NAME>GMM&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>18&lt;/SEQ_NO>  &lt;PATH>-118&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001025&lt;/ACTION_ID>  &lt;PARENT_ID>1000001024&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>MASTER&lt;/MENU_TYPE>  &lt;SEQUENCE>105&lt;/SEQUENCE>  &lt;SCREEN_NAME>MAKER&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>105&lt;/SEQ_NO>  &lt;PATH>-118-2105&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106525&lt;/ACTION_ID>  &lt;PARENT_ID>1000001025&lt;/PARENT_ID>  &lt;DISPLAY_NAME>SALESMANAGER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>genericMasterAction.do?actionPerformed=displayData&amp;amp;screenId=1100106525&amp;amp;actionId=1100106525&amp;amp;mode=M&amp;amp;actionName=QM_SALESMANAGER&amp;amp;removeSession=Y&amp;amp;searchCriteria=&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_SALESMANAGER&lt;/MENU_TYPE>  &lt;SEQUENCE>126&lt;/SEQUENCE>  &lt;SCREEN_NAME>SALESMANAGER (MAKER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION>genericMasterAction.do?actionPerformed=displaydata&amp;amp;masterId=1100106525&amp;amp;mode=M&lt;/FORWARD_READ_ACTION>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>126&lt;/SEQ_NO>  &lt;PATH>-118-2105-3126&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001026&lt;/ACTION_ID>  &lt;PARENT_ID>1000001024&lt;/PARENT_ID>  &lt;DISPLAY_NAME>AUTHOR&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>MASTER&lt;/MENU_TYPE>  &lt;SEQUENCE>305&lt;/SEQUENCE>  &lt;SCREEN_NAME>AUTHOR&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>305&lt;/SEQ_NO>  &lt;PATH>-118-2305&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001300&lt;/ACTION_ID>  &lt;PARENT_ID/>  &lt;DISPLAY_NAME>BATCH UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L1&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>313&lt;/SEQUENCE>  &lt;SCREEN_NAME>BATCH_UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>313&lt;/SEQ_NO>  &lt;PATH>-1313&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001301&lt;/ACTION_ID>  &lt;PARENT_ID>1000001300&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MAKER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>314&lt;/SEQUENCE>  &lt;SCREEN_NAME>BATCH UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>314&lt;/SEQ_NO>  &lt;PATH>-1313-2314&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>9200109154&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>AP INT GL MAP CONFIG&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=9200109154&amp;amp;actionId=9200109154&amp;amp;mode=M&amp;amp;moduleId=AP_INT_GL_MAP_CONFIG&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>AP INT GL MAP CONFIG&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106475&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BULK RECEIPT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106475&amp;amp;actionId=1100106475&amp;amp;mode=M&amp;amp;moduleId=OTC_IMD&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>BULK RECEIPT&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108861&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>INSTRUMENT MANAGEMENT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108861&amp;amp;actionId=1200108861&amp;amp;mode=M&amp;amp;moduleId=1200000300&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>INSTR MANAGEMENT MAKER/AUTHOR&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109167&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE REGISTRATION TRACKING&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109167&amp;amp;actionId=1200109167&amp;amp;mode=M&amp;amp;moduleId=LEASE_REG_DTL&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE REGISTRATION TRACKING&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106491&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>NEGATIVE LIST&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>NEGATIVE LIST UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109156&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>OFFLINE INSURANCE DETAILS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109156&amp;amp;actionId=1200109156&amp;amp;mode=M&amp;amp;moduleId=OFFLINE_INS_DTLS&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>OFFLINE INSURANCE DETAILS&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106494&lt;/ACTION_ID>  &lt;PARENT_ID>1100106491&lt;/PARENT_ID>  &lt;DISPLAY_NAME>SDN&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106494&amp;amp;actionId=1100106494&amp;amp;mode=M&amp;amp;moduleId=SDN&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>SDN NEGATIVE LIST UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315-41&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106496&lt;/ACTION_ID>  &lt;PARENT_ID>1100106491&lt;/PARENT_ID>  &lt;DISPLAY_NAME>TALIBAN&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106496&amp;amp;actionId=1100106496&amp;amp;mode=M&amp;amp;moduleId=TALIBAN&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>2&lt;/SEQUENCE>  &lt;SCREEN_NAME>TALIBAN NEGATIVE LIST UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>2&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315-42&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106493&lt;/ACTION_ID>  &lt;PARENT_ID>1100106491&lt;/PARENT_ID>  &lt;DISPLAY_NAME>AL-QAIDA&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106493&amp;amp;actionId=1100106493&amp;amp;mode=M&amp;amp;moduleId=ALQAIDA&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>AL-QAIDA NEGATIVE LIST UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315-43&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109123&lt;/ACTION_ID>  &lt;PARENT_ID>1100106491&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MCIB&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109123&amp;amp;actionId=1200109123&amp;amp;mode=M&amp;amp;moduleId=MCIB&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>MCIB&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315-43&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109105&lt;/ACTION_ID>  &lt;PARENT_ID>1100106491&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CAUTION LIST&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109105&amp;amp;actionId=1200109105&amp;amp;mode=M&amp;amp;moduleId=CAUTION_LIST&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>CAUTION LIST&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3315-4315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100010120&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PDC/ECS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100010120&amp;amp;actionId=1100010120&amp;amp;mode=M&amp;amp;moduleId=PDC&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PDC/ECS&lt;/MENU_TYPE>  &lt;SEQUENCE>316&lt;/SEQUENCE>  &lt;SCREEN_NAME>PDC/ECS&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>316&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3316&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106572&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>QT FLEET DETAIL UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106572&amp;amp;actionId=1100106572&amp;amp;mode=M&amp;amp;moduleId=BH00000195&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QT_FLEET_DETAIL&lt;/MENU_TYPE>  &lt;SEQUENCE>320&lt;/SEQUENCE>  &lt;SCREEN_NAME>QT_FLEET_DETAIL UPLOAD  (AUTOAUTH)&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>320&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3320&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106753&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE ASSET PRICE MST UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106753&amp;amp;actionId=1200106753&amp;amp;mode=M&amp;amp;moduleId=1200000199&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_ASSET_PRICE_MST&lt;/MENU_TYPE>  &lt;SEQUENCE>321&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_ASSET_PRICE_MST UPLOAD  (MAKER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>321&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3321&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106841&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE ASSET VENDOR UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106841&amp;amp;actionId=1200106841&amp;amp;mode=M&amp;amp;moduleId=1200000209&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_ASSET_VENDOR&lt;/MENU_TYPE>  &lt;SEQUENCE>321&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_ASSET_VENDOR  (MAKER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>321&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3321&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106844&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE VENDOR GSTIN UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106844&amp;amp;actionId=1200106844&amp;amp;mode=M&amp;amp;moduleId=1200000210&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_VENDOR_GSTIN&lt;/MENU_TYPE>  &lt;SEQUENCE>321&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_VENDOR_GSTIN  (MAKER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>321&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3321&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106730&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MANUAL VOUCHER UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106730&amp;amp;actionId=1100106730&amp;amp;mode=M&amp;amp;moduleId=BH00000196&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>321&lt;/SEQUENCE>  &lt;SCREEN_NAME>MANUAL VOUCHER UPLOAD (AUTOAUTH)&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>321&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3321&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108938&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER INVOICE CONFIG UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108938&amp;amp;actionId=1200108938&amp;amp;mode=M&amp;amp;moduleId=1200000213&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_CUST_INVOICE_CONFIG_DTLS&lt;/MENU_TYPE>  &lt;SEQUENCE>322&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER INVOICE CONFIG UPLOAD (MAKER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>322&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3322&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106834&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE BATTERY PRICE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106834&amp;amp;actionId=1200106834&amp;amp;mode=M&amp;amp;moduleId=1200000207&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_BATTERY_PRICE&lt;/MENU_TYPE>  &lt;SEQUENCE>322&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_BATTERY_PRICE UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>322&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3322&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106763&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE TYRE PRICE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106763&amp;amp;actionId=1200106763&amp;amp;mode=M&amp;amp;moduleId=1200000202&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_TYRE_PRICE&lt;/MENU_TYPE>  &lt;SEQUENCE>322&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_TYRE_PRICE UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>322&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3322&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108934&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MANUAL ADVICE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108934&amp;amp;actionId=1200108934&amp;amp;mode=M&amp;amp;moduleId=1200000212&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QT_MANUALADVICE&lt;/MENU_TYPE>  &lt;SEQUENCE>322&lt;/SEQUENCE>  &lt;SCREEN_NAME>MANUAL ADVICE UPLOAD (MAKER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>322&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3322&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108997&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE ASSET CESSMAPPING UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108997&amp;amp;actionId=1200108997&amp;amp;mode=M&amp;amp;moduleId=1200000303&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_ASSET_CESSMAPPING&lt;/MENU_TYPE>  &lt;SEQUENCE>323&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE ASSET CESSMAPPING UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>323&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3323&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106771&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE ASSET VARIANT UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106771&amp;amp;actionId=1200106771&amp;amp;mode=M&amp;amp;moduleId=1200000205&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_ASSET_VARIANT&lt;/MENU_TYPE>  &lt;SEQUENCE>323&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_ASSET_VARIANT UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>323&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3323&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108948&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE FC AMNT FORMULA UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108948&amp;amp;actionId=1200108948&amp;amp;mode=M&amp;amp;moduleId=1200000301&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_FC_AMNT_FORMULA&lt;/MENU_TYPE>  &lt;SEQUENCE>323&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_FC_AMNT_FORMULA UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>323&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3323&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108965&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE SERVICE CHRG AMT UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108965&amp;amp;actionId=1200108965&amp;amp;mode=M&amp;amp;moduleId=1200000302&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_SERVICE_CHRG_AMT&lt;/MENU_TYPE>  &lt;SEQUENCE>323&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE SERVICE CHRG AMT UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>323&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3323&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109004&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE VENDOR BANK UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109004&amp;amp;actionId=1200109004&amp;amp;mode=M&amp;amp;moduleId=1200000304&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_VENDOR_BANK_MASTER&lt;/MENU_TYPE>  &lt;SEQUENCE>323&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE VENDOR BANK UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>323&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3323&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109006&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE VENDOR MAPPING UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109006&amp;amp;actionId=1200109006&amp;amp;mode=M&amp;amp;moduleId=1200000305&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_VENDOR_MAPPING&lt;/MENU_TYPE>  &lt;SEQUENCE>323&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE VENDOR MAPPING UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>323&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3323&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106831&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE MAINT SLABS UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106831&amp;amp;actionId=1200106831&amp;amp;mode=M&amp;amp;moduleId=1200000206&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_MAINT_SLABS&lt;/MENU_TYPE>  &lt;SEQUENCE>324&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_MAINT_SLABS UPLOAD &lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>324&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3324&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109034&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BULK CLOSURE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109034&amp;amp;actionId=1200109034&amp;amp;mode=M&amp;amp;moduleId=BULK_CLOSURE&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>350&lt;/SEQUENCE>  &lt;SCREEN_NAME>BULK CLOSURE MAKER/AUTHOR&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>350&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3350&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106724&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CHARGE X GST UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106724&amp;amp;actionId=1100106724&amp;amp;mode=M&amp;amp;moduleId=BH00000023&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_CHARGE_X_GST&lt;/MENU_TYPE>  &lt;SEQUENCE>353&lt;/SEQUENCE>  &lt;SCREEN_NAME>CHARGE_X_GST UPLOAD  (MAKER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>353&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3353&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106838&lt;/ACTION_ID>  &lt;PARENT_ID>1000001301&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE RESIDUAL VALUE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106838&amp;amp;actionId=1200106838&amp;amp;mode=M&amp;amp;moduleId=1200000208&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_RESIDUAL_VALUE&lt;/MENU_TYPE>  &lt;SEQUENCE>996&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_RESIDUAL_VALUE UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>M&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>996&lt;/SEQ_NO>  &lt;PATH>-1313-2314-3996&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001302&lt;/ACTION_ID>  &lt;PARENT_ID>1000001300&lt;/PARENT_ID>  &lt;DISPLAY_NAME>AUTHOR&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>320&lt;/SEQUENCE>  &lt;SCREEN_NAME>BATCH UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>320&lt;/SEQ_NO>  &lt;PATH>-1313-2320&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>9200109155&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>AP INT GL MAP CONFIG&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=9200109155&amp;amp;actionId=9200109155&amp;amp;mode=A&amp;amp;moduleId=AP_INT_GL_MAP_CONFIG&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>AP INT GL MAP CONFIG&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109036&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BULK CLOSURE&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109036&amp;amp;actionId=1200109036&amp;amp;mode=A&amp;amp;moduleId=BULK_CLOSURE&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>BULK CLOSURE MAKER/AUTHOR&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109045&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BULK RECEIPT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109045&amp;amp;actionId=1200109045&amp;amp;mode=A&amp;amp;moduleId=OTC_IMD&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>BULK RECEIPT&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108863&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER RECEIPT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108863&amp;amp;actionId=1200108863&amp;amp;mode=A&amp;amp;moduleId=CUSTOMER_RECEIPT&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER RECEIPT MAKER/AUTHOR&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106754&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE ASSET PRICE MST UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106754&amp;amp;actionId=1200106754&amp;amp;mode=A&amp;amp;moduleId=1200000199&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_ASSET_PRICE_MST&lt;/MENU_TYPE>  &lt;SEQUENCE>322&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_ASSET_PRICE_MST UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>322&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3322&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100010121&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PDC/ECS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100010121&amp;amp;actionId=1100010121&amp;amp;mode=A&amp;amp;moduleId=PDC&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PDC/ECS&lt;/MENU_TYPE>  &lt;SEQUENCE>322&lt;/SEQUENCE>  &lt;SCREEN_NAME>PDC/ECS&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>322&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3322&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>9200109157&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE VENDOR BANK UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=9200109157&amp;amp;actionId=9200109157&amp;amp;mode=A&amp;amp;moduleId=1200000304&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_VENDOR_BANK_MASTER&lt;/MENU_TYPE>  &lt;SEQUENCE>323&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE VENDOR BANK UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>323&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3323&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108935&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MANUAL ADVICE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108935&amp;amp;actionId=1200108935&amp;amp;mode=A&amp;amp;moduleId=1200000212&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QT_MANUALADVICE&lt;/MENU_TYPE>  &lt;SEQUENCE>323&lt;/SEQUENCE>  &lt;SCREEN_NAME>MANUAL ADVICE UPLOAD (AUTHOR)&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>323&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3323&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106772&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE ASSET VARIANT UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106772&amp;amp;actionId=1200106772&amp;amp;mode=A&amp;amp;moduleId=1200000205&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_ASSET_VARIANT&lt;/MENU_TYPE>  &lt;SEQUENCE>324&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_ASSET_VARIANT UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>324&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3324&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108949&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE FC AMNT FORMULA UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108949&amp;amp;actionId=1200108949&amp;amp;mode=A&amp;amp;moduleId=1200000301&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_FC_AMNT_FORMULA&lt;/MENU_TYPE>  &lt;SEQUENCE>324&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_FC_AMNT_FORMULA UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>324&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3324&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108966&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE SERVICE CHRG AMT UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108966&amp;amp;actionId=1200108966&amp;amp;mode=A&amp;amp;moduleId=1200000302&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_SERVICE_CHRG_AMT&lt;/MENU_TYPE>  &lt;SEQUENCE>324&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE SERVICE CHRG AMT UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>324&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3324&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106842&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE ASSET VENDOR UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106842&amp;amp;actionId=1200106842&amp;amp;mode=A&amp;amp;moduleId=1200000209&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_ASSET_VENDOR&lt;/MENU_TYPE>  &lt;SEQUENCE>326&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_ASSET_VENDOR  (AUTHOR)&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>326&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3326&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106845&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE VENDOR GSTIN UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106845&amp;amp;actionId=1200106845&amp;amp;mode=A&amp;amp;moduleId=1200000210&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_VENDOR_GSTIN&lt;/MENU_TYPE>  &lt;SEQUENCE>326&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_VENDOR_GSTIN  (AUTHOR)&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>326&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3326&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106732&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MANUAL VOUCHER UPLOAD AUTHOR&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106732&amp;amp;actionId=1100106732&amp;amp;mode=A&amp;amp;moduleId=BH00000196&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>326&lt;/SEQUENCE>  &lt;SCREEN_NAME>MANUAL VOUCHER UPLOAD AUTHOR&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>326&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3326&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106835&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE BATTERY PRICE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106835&amp;amp;actionId=1200106835&amp;amp;mode=A&amp;amp;moduleId=1200000207&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_BATTERY_PRICE&lt;/MENU_TYPE>  &lt;SEQUENCE>327&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_BATTERY_PRICE UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>327&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3327&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106764&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE TYRE PRICE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106764&amp;amp;actionId=1200106764&amp;amp;mode=A&amp;amp;moduleId=1200000202&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_TYRE_PRICE&lt;/MENU_TYPE>  &lt;SEQUENCE>327&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_TYRE_PRICE UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>327&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3327&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106832&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE MAINT SLABS UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>BATCHUPLOADACTION.DO?ACTIONPERFORMED=BATCHUPLOAD&amp;amp;SCREENID=1200106832&amp;amp;ACTIONID=1200106832&amp;amp;MODE=A&amp;amp;MODULEID=1200000206&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_MAINT_SLABS&lt;/MENU_TYPE>  &lt;SEQUENCE>329&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_MAINT_SLABS UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>329&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3329&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109104&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>NEGATIVE LIST&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>332&lt;/SEQUENCE>  &lt;SCREEN_NAME>NEGATIVE LIST VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>332&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3332&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109106&lt;/ACTION_ID>  &lt;PARENT_ID>1200109104&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CAUTION LIST&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109106&amp;amp;actionId=1200109106&amp;amp;mode=A&amp;amp;moduleId=CAUTION_LIST&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>CAUTION LIST&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3332-4315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106725&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CHARGE X GST UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106725&amp;amp;actionId=1100106725&amp;amp;mode=A&amp;amp;moduleId=BH00000023&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_CHARGE_X_GST&lt;/MENU_TYPE>  &lt;SEQUENCE>351&lt;/SEQUENCE>  &lt;SCREEN_NAME>CHARGE_X_GST UPLOAD  (AUTHOR)&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>351&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3351&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106839&lt;/ACTION_ID>  &lt;PARENT_ID>1000001302&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE RESIDUAL VALUE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106839&amp;amp;actionId=1200106839&amp;amp;mode=A&amp;amp;moduleId=1200000208&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_RESIDUAL_VALUE&lt;/MENU_TYPE>  &lt;SEQUENCE>997&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_RESIDUAL_VALUE UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>A&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>997&lt;/SEQ_NO>  &lt;PATH>-1313-2320-3997&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1000001303&lt;/ACTION_ID>  &lt;PARENT_ID>1000001300&lt;/PARENT_ID>  &lt;DISPLAY_NAME>VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L2&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>330&lt;/SEQUENCE>  &lt;SCREEN_NAME>BATCH UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>330&lt;/SEQ_NO>  &lt;PATH>-1313-2330&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>9200109156&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>AP INT GL MAP CONFIG&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=9200109156&amp;amp;actionId=9200109156&amp;amp;mode=V&amp;amp;moduleId=AP_INT_GL_MAP_CONFIG&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>AP INT GL MAP CONFIG&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109168&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE REGISTRATION TRACKING&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109168&amp;amp;actionId=1200109168&amp;amp;mode=V&amp;amp;moduleId=LEASE_REG_DTL&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>316&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE REGISTRATION TRACKING&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>316&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3316&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109157&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>OFFLINE INSURANCE DETAILS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109157&amp;amp;actionId=1200109157&amp;amp;mode=V&amp;amp;moduleId=OFFLINE_INS_DTLS&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>316&lt;/SEQUENCE>  &lt;SCREEN_NAME>OFFLINE INSURANCE DETAILS&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>316&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3316&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106755&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE ASSET PRICE MST UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106755&amp;amp;actionId=1200106755&amp;amp;mode=V&amp;amp;moduleId=1200000199&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_ASSET_PRICE_MST&lt;/MENU_TYPE>  &lt;SEQUENCE>321&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_ASSET_PRICE_MST UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>321&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3321&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106731&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MANUAL VOUCHER UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106731&amp;amp;actionId=1100106731&amp;amp;mode=V&amp;amp;moduleId=BH00000196&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>322&lt;/SEQUENCE>  &lt;SCREEN_NAME>MANUAL VOUCHER UPLOAD (VIEWER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>322&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3322&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108940&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER INVOICE CONFIG UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108940&amp;amp;actionId=1200108940&amp;amp;mode=V&amp;amp;moduleId=1200000213&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_CUST_INVOICE_CONFIG_DTLS&lt;/MENU_TYPE>  &lt;SEQUENCE>324&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER INVOICE CONFIG UPLOAD (VIEWER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>AA&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>324&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3324&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108936&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MANUAL ADVICE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108936&amp;amp;actionId=1200108936&amp;amp;mode=V&amp;amp;moduleId=1200000212&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QT_MANUALADVICE&lt;/MENU_TYPE>  &lt;SEQUENCE>324&lt;/SEQUENCE>  &lt;SCREEN_NAME>MANUAL ADVICE UPLOAD (VIEWER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>324&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3324&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106773&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE ASSET VARIANT UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106773&amp;amp;actionId=1200106773&amp;amp;mode=V&amp;amp;moduleId=1200000205&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_ASSET_VARIANT&lt;/MENU_TYPE>  &lt;SEQUENCE>325&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_ASSET_VARIANT UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>325&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3325&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108950&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE FC AMNT FORMULA UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108950&amp;amp;actionId=1200108950&amp;amp;mode=V&amp;amp;moduleId=1200000301&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_FC_AMNT_FORMULA&lt;/MENU_TYPE>  &lt;SEQUENCE>325&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_FC_AMNT_FORMULA UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>325&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3325&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108967&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE SERVICE CHRG AMT UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108967&amp;amp;actionId=1200108967&amp;amp;mode=V&amp;amp;moduleId=1200000302&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_SERVICE_CHRG_AMT&lt;/MENU_TYPE>  &lt;SEQUENCE>325&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE SERVICE CHRG AMT UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>325&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3325&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109005&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE VENDOR BANK UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109005&amp;amp;actionId=1200109005&amp;amp;mode=V&amp;amp;moduleId=1200000304&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_VENDOR_BANK_MASTER&lt;/MENU_TYPE>  &lt;SEQUENCE>325&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE VENDOR BANK UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>325&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3325&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109007&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE VENDOR MAPPING UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109007&amp;amp;actionId=1200109007&amp;amp;mode=V&amp;amp;moduleId=1200000305&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_VENDOR_MAPPING&lt;/MENU_TYPE>  &lt;SEQUENCE>325&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE VENDOR MAPPING UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>325&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3325&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106476&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BULK RECEIPT&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106476&amp;amp;actionId=1100106476&amp;amp;mode=V&amp;amp;moduleId=OTC_IMD&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>332&lt;/SEQUENCE>  &lt;SCREEN_NAME>BULK RECEIPT&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>332&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3332&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108866&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CUSTOMER RECEIPT VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108866&amp;amp;actionId=1200108866&amp;amp;mode=V&amp;amp;moduleId=CUSTOMER_RECEIPT&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>332&lt;/SEQUENCE>  &lt;SCREEN_NAME>CUSTOMER RECEIPT VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>332&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3332&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200108862&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>INSTRUMENT MANAGEMENT VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200108861&amp;amp;actionId=1200108861&amp;amp;mode=V&amp;amp;moduleId=1200000300&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>332&lt;/SEQUENCE>  &lt;SCREEN_NAME>INSTR MANAGEMENT VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>332&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3332&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106490&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>NEGATIVE LIST&lt;/DISPLAY_NAME>  &lt;ACTION_LINK/>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>332&lt;/SEQUENCE>  &lt;SCREEN_NAME>NEGATIVE LIST VIEWER&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>332&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3332&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106484&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>RESCH VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106484&amp;amp;actionId=1100106484&amp;amp;mode=V&amp;amp;moduleId=RESCH&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>332&lt;/SEQUENCE>  &lt;SCREEN_NAME>RESCH&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>332&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3332&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106495&lt;/ACTION_ID>  &lt;PARENT_ID>1100106490&lt;/PARENT_ID>  &lt;DISPLAY_NAME>SDN&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106495&amp;amp;actionId=1100106495&amp;amp;mode=V&amp;amp;moduleId=SDN&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>1&lt;/SEQUENCE>  &lt;SCREEN_NAME>SDN NEGATIVE LIST&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>1&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3332-41&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106497&lt;/ACTION_ID>  &lt;PARENT_ID>1100106490&lt;/PARENT_ID>  &lt;DISPLAY_NAME>TALIBAN&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106497&amp;amp;actionId=1100106497&amp;amp;mode=V&amp;amp;moduleId=TALIBAN&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>2&lt;/SEQUENCE>  &lt;SCREEN_NAME>TALIBAN NEGATIVE LIST&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>2&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3332-42&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106492&lt;/ACTION_ID>  &lt;PARENT_ID>1100106490&lt;/PARENT_ID>  &lt;DISPLAY_NAME>AL-QAIDA&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106492&amp;amp;actionId=1100106492&amp;amp;mode=V&amp;amp;moduleId=ALQAIDA&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>AL-QAIDA NEGATIVE LIST&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3332-43&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109122&lt;/ACTION_ID>  &lt;PARENT_ID>1100106490&lt;/PARENT_ID>  &lt;DISPLAY_NAME>MCIB&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109122&amp;amp;actionId=1200109122&amp;amp;mode=V&amp;amp;moduleId=MCIB&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L4&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH_UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>3&lt;/SEQUENCE>  &lt;SCREEN_NAME>MCIB&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>3&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3332-43&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109107&lt;/ACTION_ID>  &lt;PARENT_ID>1100106490&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CAUTION LIST&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109107&amp;amp;actionId=1200109107&amp;amp;mode=V&amp;amp;moduleId=CAUTION_LIST&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>315&lt;/SEQUENCE>  &lt;SCREEN_NAME>CAUTION LIST&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>315&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3332-4315&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200109035&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>BULK CLOSURE VIEWER&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200109035&amp;amp;actionId=1200109035&amp;amp;mode=V&amp;amp;moduleId=BULK_CLOSURE&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>BATCH UPLOAD&lt;/MENU_TYPE>  &lt;SEQUENCE>333&lt;/SEQUENCE>  &lt;SCREEN_NAME>BULK CLOSURE&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>333&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3333&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100010128&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>PDC/ECS&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100010128&amp;amp;actionId=1100010128&amp;amp;mode=V&amp;amp;moduleId=PDC&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>PDC/ECS&lt;/MENU_TYPE>  &lt;SEQUENCE>335&lt;/SEQUENCE>  &lt;SCREEN_NAME>PDC/ECS &lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>335&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3335&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106843&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE ASSET VENDOR UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106843&amp;amp;actionId=1200106843&amp;amp;mode=V&amp;amp;moduleId=1200000209&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_ASSET_VENDOR&lt;/MENU_TYPE>  &lt;SEQUENCE>338&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_ASSET_VENDOR  (VIEWER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>338&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3338&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106846&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE VENDOR GSTIN UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106846&amp;amp;actionId=1200106846&amp;amp;mode=V&amp;amp;moduleId=1200000210&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_VENDOR_GSTIN&lt;/MENU_TYPE>  &lt;SEQUENCE>338&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_VENDOR_GSTIN  (VIEWER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>338&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3338&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106836&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE BATTERY PRICE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106836&amp;amp;actionId=1200106836&amp;amp;mode=V&amp;amp;moduleId=1200000207&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_BATTERY_PRICE&lt;/MENU_TYPE>  &lt;SEQUENCE>339&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_BATTERY_PRICE UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>339&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3339&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106765&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE TYRE PRICE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106765&amp;amp;actionId=1200106765&amp;amp;mode=V&amp;amp;moduleId=1200000202&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_TYRE_PRICE&lt;/MENU_TYPE>  &lt;SEQUENCE>339&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_TYRE_PRICE UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>339&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3339&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106833&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE MAINT SLABS UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>BATCHUPLOADACTION.DO?ACTIONPERFORMED=BATCHUPLOAD&amp;amp;SCREENID=1200106833&amp;amp;ACTIONID=1200106833&amp;amp;MODE=V&amp;amp;MODULEID=1200000206&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>MENU_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_MAINT_SLABS&lt;/MENU_TYPE>  &lt;SEQUENCE>341&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_MAINT_SLABS UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>341&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3341&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1100106726&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>CHARGE X GST UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1100106726&amp;amp;actionId=1100106726&amp;amp;mode=V&amp;amp;moduleId=BH00000023&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_CHARGE_X_GST&lt;/MENU_TYPE>  &lt;SEQUENCE>352&lt;/SEQUENCE>  &lt;SCREEN_NAME>CHARGE_X_GST UPLOAD  (VIEWER)&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>352&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3352&lt;/PATH> &lt;/ROW> &lt;ROW>  &lt;ACTION_ID>1200106840&lt;/ACTION_ID>  &lt;PARENT_ID>1000001303&lt;/PARENT_ID>  &lt;DISPLAY_NAME>LEASE RESIDUAL VALUE UPLOAD&lt;/DISPLAY_NAME>  &lt;ACTION_LINK>batchUploadAction.do?actionPerformed=batchUpload&amp;amp;screenId=1200106840&amp;amp;actionId=1200106840&amp;amp;mode=V&amp;amp;moduleId=1200000208&lt;/ACTION_LINK>  &lt;ACTION_TARGET/>  &lt;ACTION_VISIBILITY>N&lt;/ACTION_VISIBILITY>  &lt;ACTION_STYLE>menu_L3&lt;/ACTION_STYLE>  &lt;MENU_TYPE>QM_LEASE_RESIDUAL_VALUE&lt;/MENU_TYPE>  &lt;SEQUENCE>998&lt;/SEQUENCE>  &lt;SCREEN_NAME>LEASE_RESIDUAL_VALUE UPLOAD&lt;/SCREEN_NAME>  &lt;AUTHMODE>V&lt;/AUTHMODE>  &lt;PRIVILEGE_ID>E&lt;/PRIVILEGE_ID>  &lt;FORWARD_READ_ACTION/>  &lt;MENU_LOCATION>HOME&lt;/MENU_LOCATION>  &lt;DISPLAY_ACTIVITY>N&lt;/DISPLAY_ACTIVITY>  &lt;APPLICATION_URL/>  &lt;REQUEST_PAGE/>  &lt;SEQ_NO>998&lt;/SEQ_NO>  &lt;PATH>-1313-2330-3998&lt;/PATH> &lt;/ROW>&lt;/ROWSET>&quot;;
		           // alert(menuDataXml)
					setMenuData(menuDataXml);
					renderItemsInto(&quot;verticalMenu&quot;);
		
		
					
				
			
		
	

					

	
		

		
		
			




  
 
    
  miFIN
    
	
	
	    
	
	
	

  
  
  
  
	
  
   	  
			 		 
						  
						  
						  
					  
		
 
 




		
	
	      	
   	
   		
		
		
		
		
			
							CHARGE DETAILS
							
								
									 
									
									
									
			
		
		
				
				
		 
         
         
                  
         
         
         
          
          
          
          
          
          
          
             
        
         
         
		
				
				
								
				
		
		
			
		      	
					
						
							
					 
						 Charge Type
						 Dr/Cr Note
						 Misc. Bill
						
						 Invoice No
						 Charge Id
						 Charge Amount
						 Charge/Invoice Date
						Bill Start Date
						Bill End Date
						 Payment Due Date
						
						 Maker Remarks  
				        
				        
							
				        
				        
						
					    	 Send To Author  
						
						
					
				
			
			
				
				SelectReceivablePayable SelectPRO RATA INTEREST        
			
		
		         
	     
	     
	     
	     
	
	              




	
		
		Qualtech Consultants Pvt. Ltd.
		
		
		VERSION 1.0.1.295
		
			
			   
				 
		
	
		
 


	var objText = null;			
	                  
	function refreshCustomerServiceInLos()
	{
	  var pageURL = $(location).attr(&quot;href&quot;);
       var pathName = pageURL.substring(pageURL.lastIndexOf(&quot; , &quot;'&quot; , &quot;/&quot; , &quot;'&quot; , &quot;) + 1,Number(pageURL.length));
       $(location).attr(&quot; , &quot;'&quot; , &quot;href&quot; , &quot;'&quot; , &quot;, pathName);
	 } 

$(window).on(&quot; , &quot;'&quot; , &quot;load&quot; , &quot;'&quot; , &quot;, function(){
	if(!$(&quot;div&quot;).hasClass(&quot;menu_tab&quot;)){
	
		$(&quot;body:not(.body_searchlist)&quot;).addClass(&quot;menuHavingBody&quot;);
		//$(&quot;body&quot;).not(&quot;.body_searchlist&quot;).addClass(&quot;menuHavingBody&quot;);
		//$(&quot;body&quot;).addClass(&quot;menuHavingBody&quot;);
	}
	if($(&quot;.subheaderSection div div:nth-of-type(2)&quot;).text()!==&quot;&quot;){
	$(&quot;.slimScrollDiv&quot;).addClass(&quot;responsiveMenu&quot;);
	}	
	$(&quot;#securityCheckList&quot;).next(&quot;table&quot;).find(&quot;td:nth-of-type(1)&quot;).css(&quot;width&quot;,&quot;5%&quot;);
	$(&quot;#secContainer #securityCheckList tr:not(&quot; , &quot;'&quot; , &quot;:nth-child(1)&quot; , &quot;'&quot; , &quot;) td:nth-child(9)&quot;).css(&quot;text-align&quot;,&quot;right&quot;);
	if($(document).height()>($(window).height())){
	$(&quot;.text_footer&quot;).addClass(&quot;posrell&quot;);
	}
	$(&quot;a,.blueBotton&quot;).click(function(){
	if($(document).height()>($(window).height())){$(&quot;.text_footer&quot;).addClass(&quot;posrell&quot;);}
	});

});


function disableAllElementsAjax()
		    {
		    	
		  	   
	                  for(count=0; count &lt; document.forms[0].elements.length; count+=1)
				        {
				        	theelement = document.forms[0].elements[count];
				            if(theelement.name != null &amp;&amp; theelement.name != &quot;btn_one&quot; &amp;&amp;theelement.name !=&quot;CreditOfficerHistory&quot; &amp;&amp;theelement.name != &quot;btnCam&quot; &amp;&amp;theelement.name != &quot;btnSanction&quot;)
				            {
				            	theelement.disabled = true;
				            }
				        }
				      
			  
		    }           

$(window).on(&quot; , &quot;'&quot; , &quot;load&quot; , &quot;'&quot; , &quot;, function(){
	if($(&quot;.subheaderSection div div:nth-of-type(2)&quot;).text()!==&quot;&quot;){
		$(&quot;.slimScrollDiv&quot;).addClass(&quot;responsiveMenu&quot;);
	}	
	$(&quot;.menu_tab&quot;).parents(&quot;body&quot;).addClass(&quot;menuHavingBody&quot;);
	//1.0.0.1 start
	if(!$(&quot;textarea&quot;).prop(&quot;disabled&quot;))
	{
			$(&quot;textarea&quot;).wrap(&quot; , &quot;'&quot; , &quot;&lt;span class=&quot;textarea-span&quot; style=&quot;position: relative;float: left;width: 100%;&quot;>&lt;/span>&quot; , &quot;'&quot; , &quot;);
			$(&quot;.textarea-span&quot;).append(&quot; , &quot;'&quot; , &quot;&lt;span class=&quot;edit-textarea&quot;>&lt;/span>&quot; , &quot;'&quot; , &quot;);
	
	} // 1.0.0.1 end	
	$(document).on(&quot;click&quot;,&quot;.edit-textarea&quot;, function(){
			objText = $(this);
			//Start Added by 1.0.0.2
			if(objText.prev(&quot;textarea&quot;).prop(&quot;disabled&quot;))
				return;
			//End Added by 1.0.0.2
			$(&quot;.justbeforeform&quot;).remove();
			$(&quot;.textarea-popup-content&quot;).remove();
			var eachtareacontent = $(this).prev(&quot;textarea&quot;).val();
			$(&quot;body&quot;).append(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;justbeforeform&quot;>&lt;/div> &lt;div class=&quot;textarea-popup-content&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;);
			$(&quot;.textarea-popup-content&quot;).wrapInner(&quot; , &quot;'&quot; , &quot;&lt;div class=&quot;div_popup&quot; contenteditable=&quot;true&quot;>&lt;/div>&quot; , &quot;'&quot; , &quot;);
			$(&quot;.textarea-popup-content&quot;).animate({top: &quot;100px&quot;});
			
			$(&quot;.div_popup&quot;).text(eachtareacontent);
			$(&quot;.div_popup&quot;).after(&quot; , &quot;'&quot; , &quot;&lt;input type=&quot;button&quot; class=&quot;ok_btn&quot; value=&quot;Close&quot; style=&quot;float:right&quot; />&quot; , &quot;'&quot; , &quot;);
			$(&quot; , &quot;'&quot; , &quot;html, body&quot; , &quot;'&quot; , &quot;).animate({scrollTop : 0},800);
		
			$(document).on(&quot;click&quot;,&quot;.ok_btn&quot;,function(){
			var tyu=$(&quot;.div_popup&quot;).text();
		
				objText.prev(&quot;textarea&quot;).val(tyu);
				$(&quot;.ok_btn&quot;).parent(&quot;.textarea-popup-content&quot;).siblings(&quot;.justbeforeform&quot;).fadeOut(0);
				$(&quot;.ok_btn&quot;).parent(&quot;.textarea-popup-content&quot;).animate({top: &quot;-600px&quot;});
				$(&quot;.ok_btn&quot;).parent(&quot;.textarea-popup-content&quot;).siblings(&quot;.justbeforeform&quot;).css(&quot;display&quot;,&quot;none&quot;);
				//$(&quot;.ok_btn&quot;).parent(&quot;.textarea-popup-content&quot;).css(&quot;display&quot;,&quot;none&quot;);
			});
		});
});

       

	



  

/html[1]/body[@class=&quot;menuHavingBody&quot;]&quot;))]</value>
      <webElementGuid>b62abc06-641d-4ba5-8d52-0b763152be7e</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
